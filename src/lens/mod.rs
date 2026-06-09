mod account;
mod graphql;
mod models;
mod posts;
mod queries;
mod source;

use reqwest::Url;

use crate::{SourceError, TransportConfig, parse::common::parse_http_url, transport::Transport};

const API_URL: &str = "https://api.lens.xyz/graphql";

#[derive(Clone)]
pub struct Lens {
    api_url: Url,
    username: String,
    transport: Transport,
}

impl Lens {
    pub fn new(username: impl AsRef<str>) -> Result<Self, SourceError> {
        Self::new_with_config(username, TransportConfig::default())
    }

    pub fn new_with_config(
        username: impl AsRef<str>,
        config: TransportConfig,
    ) -> Result<Self, SourceError> {
        let username = username
            .as_ref()
            .trim_start_matches('@')
            .strip_prefix("lens/")
            .unwrap_or_else(|| username.as_ref().trim_start_matches('@'))
            .to_string();
        if username.is_empty() {
            return Err(SourceError::InvalidIdentifier(username));
        }
        Ok(Self {
            api_url: Url::parse(API_URL).unwrap(),
            username,
            transport: Transport::new(config)?,
        })
    }

    pub fn with_api_url(mut self, url: impl AsRef<str>) -> Result<Self, SourceError> {
        self.api_url = parse_http_url(url.as_ref())?;
        Ok(self)
    }
}
