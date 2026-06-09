use reqwest::Url;

use crate::{
    SourceError, TransportConfig, parse::common::normalize_base_url, transport::Transport,
};

use super::{API_URL, SYNDICATION_URL, Twitter};

impl Twitter {
    pub fn new() -> Result<Self, SourceError> {
        Self::new_with_config(TransportConfig::default())
    }

    pub fn new_with_config(config: TransportConfig) -> Result<Self, SourceError> {
        Ok(Self {
            api_url: Url::parse(API_URL).unwrap(),
            syndication_url: Url::parse(SYNDICATION_URL).unwrap(),
            transport: Transport::new(config)?,
            authenticated: false,
        })
    }

    pub fn with_bearer_token(mut self, token: impl Into<String>) -> Self {
        let token = token.into();
        self.authenticated = !token.trim().is_empty();
        self.transport = self.transport.with_bearer_token(token);
        self
    }

    pub fn with_access_token(self, token: impl Into<String>) -> Self {
        self.with_bearer_token(token)
    }

    pub fn with_api_url(mut self, url: impl AsRef<str>) -> Result<Self, SourceError> {
        self.api_url = normalize_base_url(url.as_ref())?;
        Ok(self)
    }

    pub fn with_syndication_url(mut self, url: impl AsRef<str>) -> Result<Self, SourceError> {
        self.syndication_url = normalize_base_url(url.as_ref())?;
        Ok(self)
    }

    pub fn api_url(&self) -> &Url {
        &self.api_url
    }

    pub fn syndication_url(&self) -> &Url {
        &self.syndication_url
    }

    pub fn with_base_url(self, url: impl AsRef<str>) -> Result<Self, SourceError> {
        self.with_api_url(url)
    }

    pub fn base_url(&self) -> &Url {
        self.api_url()
    }

    pub(super) fn api_endpoint(&self, path: &[&str]) -> Url {
        let mut url = self.api_url.clone();
        url.path_segments_mut().unwrap().extend(path);
        url
    }

    pub(super) fn syndication_endpoint(&self, username: &str) -> Url {
        let mut url = self.syndication_url.clone();
        url.path_segments_mut().unwrap().push(username);
        url
    }
}

impl Default for Twitter {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

impl std::fmt::Debug for Twitter {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Twitter")
            .field("api_url", &self.api_url)
            .field("syndication_url", &self.syndication_url)
            .field("authenticated", &self.authenticated)
            .finish()
    }
}
