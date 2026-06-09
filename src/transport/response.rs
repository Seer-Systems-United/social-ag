use std::time::{Duration, Instant};

use reqwest::{
    StatusCode,
    blocking::Response,
    header::{CACHE_CONTROL, ETAG, LAST_MODIFIED, RETRY_AFTER},
};

use crate::SourceError;

use super::{Transport, state::CachedResponse};

impl Transport {
    pub(super) fn handle_response(
        &self,
        response: Response,
        host: &str,
        key: &str,
    ) -> Result<Vec<u8>, SourceError> {
        let status = response.status();
        if status == StatusCode::NOT_MODIFIED {
            self.record_success(host);
            return self
                .cached_body(key)
                .ok_or_else(|| SourceError::InvalidResponse("304 without cache".into()));
        }
        if let Some(error) = self.status_error(&response) {
            return Err(error);
        }

        let headers = response.headers().clone();
        let body = response.bytes()?.to_vec();
        self.record_success(host);
        let no_store = headers
            .get(CACHE_CONTROL)
            .and_then(|value| value.to_str().ok())
            .is_some_and(|value| value.contains("no-store"));
        let mut state = self.state.0.lock().unwrap();
        if no_store {
            state.cache.remove(key);
        } else {
            state.cache.insert(
                key.into(),
                CachedResponse {
                    body: body.clone(),
                    etag: headers.get(ETAG).cloned(),
                    last_modified: headers.get(LAST_MODIFIED).cloned(),
                    expires_at: Instant::now() + cache_ttl(&headers, self.config.cache_ttl),
                },
            );
        }
        Ok(body)
    }

    fn status_error(&self, response: &Response) -> Option<SourceError> {
        let status = response.status();
        let error = match status {
            StatusCode::UNAUTHORIZED => SourceError::AuthenticationRequired,
            StatusCode::FORBIDDEN => SourceError::Blocked { status },
            StatusCode::NOT_FOUND => {
                self.record_success(response.url().host_str().unwrap_or_default());
                return Some(SourceError::NotFound);
            }
            StatusCode::TOO_MANY_REQUESTS => SourceError::RateLimited {
                retry_after: retry_after(response),
            },
            status if status.is_server_error() => SourceError::Unavailable { status },
            status if !status.is_success() => SourceError::Http { status },
            _ => return None,
        };
        self.record_failure(response.url().host_str().unwrap_or_default());
        Some(error)
    }
}

fn retry_after(response: &Response) -> Option<Duration> {
    let value = response.headers().get(RETRY_AFTER)?.to_str().ok()?;
    value.parse().ok().map(Duration::from_secs).or_else(|| {
        chrono::DateTime::parse_from_rfc2822(value)
            .ok()?
            .with_timezone(&chrono::Utc)
            .signed_duration_since(chrono::Utc::now())
            .to_std()
            .ok()
    })
}

fn cache_ttl(headers: &reqwest::header::HeaderMap, default: Duration) -> Duration {
    headers
        .get(CACHE_CONTROL)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.split(',').find_map(max_age))
        .map(Duration::from_secs)
        .unwrap_or(default)
}

fn max_age(directive: &str) -> Option<u64> {
    directive.trim().strip_prefix("max-age=")?.parse().ok()
}
