use std::{thread, time::Duration};

use reqwest::{
    Url,
    blocking::RequestBuilder,
    header::{ACCEPT, AUTHORIZATION, IF_MODIFIED_SINCE, IF_NONE_MATCH},
};

use crate::SourceError;

use super::Transport;

impl Transport {
    pub(super) fn get_bytes(&self, url: Url, accept: &str) -> Result<Vec<u8>, SourceError> {
        let host = url
            .host_str()
            .ok_or_else(|| SourceError::InvalidUrl(url.to_string()))?
            .to_string();
        let key = format!("{accept} {url}");
        self.ensure_circuit_closed(&host)?;
        if let Some(body) = self.fresh_cached_body(&key) {
            return Ok(body);
        }

        let mut attempt = 0;
        loop {
            let request = self.request(&url, accept, &key);
            let permit = self.acquire_host(&host);
            let result = request.send();
            drop(permit);
            match result {
                Ok(response) => match self.handle_response(response, &host, &key) {
                    Ok(body) => return Ok(body),
                    Err(SourceError::RateLimited { retry_after })
                        if attempt < self.config.max_retries =>
                    {
                        attempt += 1;
                        thread::sleep(self.retry_delay(attempt, retry_after));
                    }
                    Err(SourceError::Unavailable { .. }) if attempt < self.config.max_retries => {
                        attempt += 1;
                        thread::sleep(self.retry_delay(attempt, None));
                    }
                    Err(error) => return Err(error),
                },
                Err(error) if attempt < self.config.max_retries && error.is_timeout() => {
                    attempt += 1;
                    self.record_failure(&host);
                    thread::sleep(self.retry_delay(attempt, None));
                }
                Err(error) => {
                    self.record_failure(&host);
                    return Err(SourceError::Request(error));
                }
            }
        }
    }

    fn request(&self, url: &Url, accept: &str, key: &str) -> RequestBuilder {
        let mut request = self.client.get(url.clone()).header(ACCEPT, accept);
        if let Some(token) = &self.bearer_token {
            request = request.header(AUTHORIZATION, format!("Bearer {token}"));
        }
        if let Some(cached) = self.state.0.lock().unwrap().cache.get(key) {
            if let Some(etag) = &cached.etag {
                request = request.header(IF_NONE_MATCH, etag.clone());
            }
            if let Some(modified) = &cached.last_modified {
                request = request.header(IF_MODIFIED_SINCE, modified.clone());
            }
        }
        request
    }

    fn retry_delay(&self, attempt: usize, retry_after: Option<Duration>) -> Duration {
        retry_after
            .unwrap_or_else(|| Duration::from_millis(250 * 2_u64.pow(attempt as u32 - 1)))
            .min(self.config.max_retry_delay)
    }
}
