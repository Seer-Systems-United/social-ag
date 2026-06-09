mod account;
mod models;
mod posts;
mod source;

use reqwest::Url;

use crate::{
    SourceError, TransportConfig, parse::common::normalize_base_url, transport::Transport,
};

const API_URL: &str = "https://hacker-news.firebaseio.com/";

#[derive(Clone)]
struct Parser {
    base_url: Url,
    transport: Transport,
}

impl Parser {
    fn new(config: TransportConfig) -> Result<Self, SourceError> {
        Ok(Self {
            base_url: normalize_base_url(API_URL)?,
            transport: Transport::new(config)?,
        })
    }
}

#[derive(Clone)]
pub struct HackerNews {
    parser: Parser,
}

impl HackerNews {
    pub fn new() -> Result<Self, SourceError> {
        Self::new_with_config(TransportConfig::default())
    }

    pub fn new_with_config(config: TransportConfig) -> Result<Self, SourceError> {
        Ok(Self {
            parser: Parser::new(config)?,
        })
    }

    pub fn with_base_url(mut self, base_url: impl AsRef<str>) -> Result<Self, SourceError> {
        self.parser.base_url = normalize_base_url(base_url.as_ref())?;
        Ok(self)
    }

    pub fn base_url(&self) -> &Url {
        &self.parser.base_url
    }
}

impl Default for HackerNews {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

impl std::fmt::Debug for HackerNews {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("HackerNews")
            .field("base_url", &self.parser.base_url)
            .finish()
    }
}
