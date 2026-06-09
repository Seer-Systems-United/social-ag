mod catalog;
mod models;
mod posts;
mod source;

use reqwest::Url;

use crate::{
    SourceError, TransportConfig, User, parse::common::parse_http_url, transport::Transport,
};

#[derive(Clone)]
pub struct Bandcamp {
    artist_url: Url,
    user: User,
    transport: Transport,
}

impl Bandcamp {
    pub fn new(username: impl AsRef<str>) -> Result<Self, SourceError> {
        Self::new_with_config(username, TransportConfig::default())
    }

    pub fn new_with_config(
        username: impl AsRef<str>,
        config: TransportConfig,
    ) -> Result<Self, SourceError> {
        let username = username.as_ref().trim().to_string();
        if username.is_empty() {
            return Err(SourceError::InvalidIdentifier(username));
        }
        let artist_url = parse_http_url(&format!("https://{username}.bandcamp.com/music"))?;
        Ok(Self {
            user: User {
                id: username.clone(),
                username,
                display_name: None,
                profile_url: artist_url.to_string(),
            },
            artist_url,
            transport: Transport::new(config)?,
        })
    }

    pub fn with_artist_url(mut self, url: impl AsRef<str>) -> Result<Self, SourceError> {
        self.artist_url = parse_http_url(url.as_ref())?;
        Ok(self)
    }
}
