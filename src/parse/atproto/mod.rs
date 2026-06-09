mod account;
mod convert;
mod feed;
mod identifier;

use reqwest::Url;

use crate::{
    SourceError, TransportConfig, parse::common::normalize_base_url, transport::Transport,
};

#[derive(Clone)]
pub(crate) struct Parser {
    pub(super) service_url: Url,
    pub(super) web_url: Url,
    pub(super) transport: Transport,
}

impl Parser {
    pub(crate) fn new(
        service_url: impl AsRef<str>,
        web_url: impl AsRef<str>,
        config: TransportConfig,
    ) -> Result<Self, SourceError> {
        Ok(Self {
            service_url: normalize_base_url(service_url.as_ref())?,
            web_url: normalize_base_url(web_url.as_ref())?,
            transport: Transport::new(config)?,
        })
    }

    pub(crate) fn service_url(&self) -> &Url {
        &self.service_url
    }

    pub(super) fn xrpc_url(&self, method: &str) -> Url {
        self.service_url.join(&format!("xrpc/{method}")).unwrap()
    }
}

impl std::fmt::Debug for Parser {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("AtProtoParser")
            .field("service_url", &self.service_url)
            .field("web_url", &self.web_url)
            .finish_non_exhaustive()
    }
}
