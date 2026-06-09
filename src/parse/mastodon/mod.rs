mod account;
mod models;
mod status;

use reqwest::Url;

use crate::{
    SourceError, SourceQuirk, TransportConfig, parse::common::normalize_base_url,
    transport::Transport,
};

use models::ApiInstance;

pub type MastodonError = SourceError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstanceMetadata {
    pub version: String,
    pub api_version: Option<u64>,
}

#[derive(Clone)]
pub(crate) struct Parser {
    pub(super) instance_url: Url,
    pub(super) transport: Transport,
    pub(super) quirks: &'static [SourceQuirk],
}

impl Parser {
    pub(crate) fn new(
        instance_url: impl AsRef<str>,
        config: TransportConfig,
        quirks: &'static [SourceQuirk],
    ) -> Result<Self, SourceError> {
        Ok(Self {
            instance_url: normalize_base_url(instance_url.as_ref())?,
            transport: Transport::new(config)?,
            quirks,
        })
    }

    pub(crate) fn with_access_token(mut self, token: impl Into<String>) -> Self {
        self.transport = self.transport.with_bearer_token(token);
        self
    }

    pub(crate) fn instance_url(&self) -> &Url {
        &self.instance_url
    }

    pub(crate) fn probe_instance(&self) -> Result<InstanceMetadata, SourceError> {
        let metadata: ApiInstance = self
            .transport
            .get_json(self.api_versioned_url("v2", &["instance"]))?;
        Ok(InstanceMetadata {
            version: metadata.version,
            api_version: metadata.api_versions.and_then(|value| value.mastodon),
        })
    }

    pub(super) fn api_url(&self, path: &[&str]) -> Url {
        self.api_versioned_url("v1", path)
    }

    fn api_versioned_url(&self, version: &str, path: &[&str]) -> Url {
        let mut url = self.instance_url.join(&format!("api/{version}/")).unwrap();
        let mut segments = url.path_segments_mut().unwrap();
        segments.pop_if_empty();
        segments.extend(path);
        drop(segments);
        url
    }
}

impl std::fmt::Debug for Parser {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("MastodonParser")
            .field("instance_url", &self.instance_url)
            .field("quirks", &self.quirks)
            .finish_non_exhaustive()
    }
}
