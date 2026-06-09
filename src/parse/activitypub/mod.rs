mod account;
mod feed;
mod models;
mod outbox;
mod post;
mod value;

use reqwest::Url;

use crate::{
    SourceError, TransportConfig, parse::common::normalize_base_url, transport::Transport,
};

pub(super) const ACTIVITY_ACCEPT: &str = "application/activity+json, application/ld+json; profile=\"https://www.w3.org/ns/activitystreams\"";
pub(super) const PUBLIC_AUDIENCE: &str = "https://www.w3.org/ns/activitystreams#Public";

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
}

impl std::fmt::Debug for Parser {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ActivityPubParser")
            .field("instance_url", &self.instance_url)
            .finish_non_exhaustive()
    }
}
