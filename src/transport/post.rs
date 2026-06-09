use std::thread;

use reqwest::{
    Url,
    header::{ACCEPT, AUTHORIZATION},
};
use serde::{Serialize, de::DeserializeOwned};

use crate::SourceError;

use super::Transport;

impl Transport {
    pub(crate) fn post_json<T, B>(&self, url: Url, body: &B) -> Result<T, SourceError>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let serialized = serde_json::to_vec(body)?;
        let host = url
            .host_str()
            .ok_or_else(|| SourceError::InvalidUrl(url.to_string()))?
            .to_string();
        let key = format!("POST {url} {}", String::from_utf8_lossy(&serialized));
        self.ensure_circuit_closed(&host)?;
        if let Some(body) = self.fresh_cached_body(&key) {
            return serde_json::from_slice(&body).map_err(SourceError::from);
        }
        let mut attempt = 0;
        loop {
            let mut request = self
                .client
                .post(url.clone())
                .header(ACCEPT, "application/json")
                .json(body);
            if let Some(token) = &self.bearer_token {
                request = request.header(AUTHORIZATION, format!("Bearer {token}"));
            }
            let permit = self.acquire_host(&host);
            let result = request.send();
            drop(permit);
            match result {
                Ok(response) => match self.handle_response(response, &host, &key) {
                    Ok(bytes) => return serde_json::from_slice(&bytes).map_err(SourceError::from),
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
}
