mod models;
mod posts;
mod source;

use reqwest::Url;

use crate::{
    SourceError, TransportConfig, parse::common::normalize_base_url, transport::Transport,
};

const API_URL: &str = "https://rutube.ru/api/video/person/";

#[derive(Clone)]
pub struct Rutube {
    api_url: Url,
    channel_id: String,
    transport: Transport,
}

impl Rutube {
    pub fn new(channel_id: impl AsRef<str>) -> Result<Self, SourceError> {
        Self::new_with_config(channel_id, TransportConfig::default())
    }

    pub fn new_with_config(
        channel_id: impl AsRef<str>,
        config: TransportConfig,
    ) -> Result<Self, SourceError> {
        let channel_id = channel_id.as_ref().trim_matches('/').to_string();
        if channel_id.is_empty() {
            return Err(SourceError::InvalidIdentifier(channel_id));
        }
        Ok(Self {
            api_url: Url::parse(API_URL).unwrap(),
            channel_id,
            transport: Transport::new(config)?,
        })
    }

    pub fn with_api_url(mut self, url: impl AsRef<str>) -> Result<Self, SourceError> {
        self.api_url = normalize_base_url(url.as_ref())?;
        Ok(self)
    }

    pub fn api_url(&self) -> &Url {
        &self.api_url
    }

    pub(super) fn channel_url(&self) -> Url {
        let mut url = self.api_url.clone();
        url.path_segments_mut()
            .unwrap()
            .extend([self.channel_id.as_str(), ""]);
        url.query_pairs_mut().append_pair("format", "json");
        url
    }
}

impl std::fmt::Debug for Rutube {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Rutube")
            .field("api_url", &self.api_url)
            .field("channel_id", &self.channel_id)
            .finish_non_exhaustive()
    }
}
