mod cache;
mod circuit;
mod config;
mod post;
mod request;
mod response;
mod state;

use std::sync::{Arc, Condvar, Mutex};

use reqwest::{Url, blocking::Client};
use serde::de::DeserializeOwned;

use crate::SourceError;

pub use config::TransportConfig;
use state::TransportState;

#[derive(Clone)]
pub(crate) struct Transport {
    client: Client,
    config: TransportConfig,
    bearer_token: Option<String>,
    state: Arc<(Mutex<TransportState>, Condvar)>,
}

impl Transport {
    pub(crate) fn new(config: TransportConfig) -> Result<Self, SourceError> {
        let client = Client::builder()
            .timeout(config.timeout)
            .user_agent(&config.user_agent)
            .build()?;
        Ok(Self {
            client,
            config,
            bearer_token: None,
            state: Arc::new((Mutex::new(TransportState::default()), Condvar::new())),
        })
    }

    pub(crate) fn with_bearer_token(mut self, token: impl Into<String>) -> Self {
        let token = token.into();
        self.bearer_token = (!token.trim().is_empty()).then_some(token);
        self.state = Arc::new((Mutex::new(TransportState::default()), Condvar::new()));
        self
    }

    pub(crate) fn get_json<T>(&self, url: Url) -> Result<T, SourceError>
    where
        T: DeserializeOwned,
    {
        let body = self.get_bytes(url, "application/json")?;
        serde_json::from_slice(&body).map_err(SourceError::from)
    }

    pub(crate) fn get_text(&self, url: Url, accept: &str) -> Result<String, SourceError> {
        let body = self.get_bytes(url, accept)?;
        String::from_utf8(body).map_err(|error| SourceError::InvalidResponse(error.to_string()))
    }
}
