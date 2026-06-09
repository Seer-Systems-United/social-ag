mod page;
mod source;

use reqwest::Url;

use crate::{
    SourceError, TransportConfig, User, parse::common::parse_http_url, transport::Transport,
};

const BASE_URL: &str = "https://t.me/s/";

#[derive(Clone)]
pub struct Telegram {
    fetch_url: Url,
    user: User,
    transport: Transport,
}

impl Telegram {
    pub fn new(username: impl AsRef<str>) -> Result<Self, SourceError> {
        Self::new_with_config(username, TransportConfig::default())
    }

    pub fn new_with_config(
        username: impl AsRef<str>,
        config: TransportConfig,
    ) -> Result<Self, SourceError> {
        let username = username.as_ref().trim_start_matches('@').to_string();
        let fetch_url = parse_http_url(&format!("{BASE_URL}{username}"))?;
        Ok(Self {
            user: User {
                id: username.clone(),
                username: username.clone(),
                display_name: None,
                profile_url: format!("https://t.me/{username}"),
            },
            fetch_url,
            transport: Transport::new(config)?,
        })
    }

    pub fn with_fetch_url(mut self, url: impl AsRef<str>) -> Result<Self, SourceError> {
        self.fetch_url = parse_http_url(url.as_ref())?;
        Ok(self)
    }

    pub fn profile_url(&self) -> &Url {
        &self.fetch_url
    }
}

impl std::fmt::Debug for Telegram {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Telegram")
            .field("fetch_url", &self.fetch_url)
            .field("user", &self.user)
            .finish_non_exhaustive()
    }
}
