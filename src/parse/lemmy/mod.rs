mod account;
mod models;
mod posts;

use reqwest::Url;

use crate::{
    SourceError, TransportConfig, parse::common::normalize_base_url, transport::Transport,
};

#[derive(Clone)]
pub(crate) struct Parser {
    pub(super) instance_url: Url,
    pub(super) transport: Transport,
}

impl Parser {
    pub(crate) fn new(
        instance_url: impl AsRef<str>,
        config: TransportConfig,
    ) -> Result<Self, SourceError> {
        Ok(Self {
            instance_url: normalize_base_url(instance_url.as_ref())?,
            transport: Transport::new(config)?,
        })
    }

    pub(crate) fn with_access_token(mut self, token: impl Into<String>) -> Self {
        self.transport = self.transport.with_bearer_token(token);
        self
    }

    pub(crate) fn instance_url(&self) -> &Url {
        &self.instance_url
    }

    pub(super) fn api_url(&self, endpoint: &str) -> Url {
        self.instance_url
            .join(&format!("api/v3/{endpoint}"))
            .unwrap()
    }
}

impl std::fmt::Debug for Parser {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("LemmyParser")
            .field("instance_url", &self.instance_url)
            .finish_non_exhaustive()
    }
}
