mod account;
mod models;
mod posts;
mod source;

use reqwest::Url;

use crate::{
    Capability, SourceError, TransportConfig, parse::common::normalize_base_url,
    transport::Transport,
};

const API_URL: &str = "https://graph.facebook.com/v25.0/";
const CAPABILITIES: &[Capability] = &[
    Capability::LookupUserById,
    Capability::LookupUserByUsername,
    Capability::FetchUserPosts,
];

#[derive(Clone)]
pub struct Facebook {
    api_url: Url,
    transport: Transport,
    authenticated: bool,
}

impl Facebook {
    pub fn new() -> Self {
        Self::new_with_config(TransportConfig::default()).unwrap()
    }

    pub fn new_with_config(config: TransportConfig) -> Result<Self, SourceError> {
        Ok(Self {
            api_url: Url::parse(API_URL).unwrap(),
            transport: Transport::new(config)?,
            authenticated: false,
        })
    }

    pub fn with_access_token(mut self, token: impl Into<String>) -> Self {
        let token = token.into();
        self.authenticated = !token.trim().is_empty();
        self.transport = self.transport.with_bearer_token(token);
        self
    }

    pub fn with_api_url(mut self, url: impl AsRef<str>) -> Result<Self, SourceError> {
        self.api_url = normalize_base_url(url.as_ref())?;
        Ok(self)
    }

    pub fn api_url(&self) -> &Url {
        &self.api_url
    }

    pub(super) fn endpoint(&self, path: &[&str]) -> Url {
        let mut url = self.api_url.clone();
        url.path_segments_mut().unwrap().extend(path);
        url
    }

    pub(super) fn require_authentication(&self) -> Result<(), SourceError> {
        self.authenticated
            .then_some(())
            .ok_or(SourceError::AuthenticationRequired)
    }
}

impl Default for Facebook {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for Facebook {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Facebook")
            .field("api_url", &self.api_url)
            .field("authenticated", &self.authenticated)
            .finish()
    }
}
