mod account;
mod models;
mod posts;
mod source;

use reqwest::Url;

use crate::{
    SourceError, TransportConfig, parse::common::normalize_base_url, transport::Transport,
};

const USERS_URL: &str = "https://users.roblox.com/v1/users/";
const GAMES_URL: &str = "https://games.roblox.com/v2/users/";

#[derive(Clone)]
pub struct Roblox {
    users_url: Url,
    games_url: Url,
    user_id: String,
    transport: Transport,
}

impl Roblox {
    pub fn new(user_id: impl AsRef<str>) -> Result<Self, SourceError> {
        Self::new_with_config(user_id, TransportConfig::default())
    }

    pub fn new_with_config(
        user_id: impl AsRef<str>,
        config: TransportConfig,
    ) -> Result<Self, SourceError> {
        let user_id = user_id.as_ref().trim().to_string();
        if user_id.is_empty() || !user_id.chars().all(|character| character.is_ascii_digit()) {
            return Err(SourceError::InvalidIdentifier(user_id));
        }
        Ok(Self {
            users_url: Url::parse(USERS_URL).unwrap(),
            games_url: Url::parse(GAMES_URL).unwrap(),
            user_id,
            transport: Transport::new(config)?,
        })
    }

    pub fn with_api_urls(
        mut self,
        users_url: impl AsRef<str>,
        games_url: impl AsRef<str>,
    ) -> Result<Self, SourceError> {
        self.users_url = normalize_base_url(users_url.as_ref())?;
        self.games_url = normalize_base_url(games_url.as_ref())?;
        Ok(self)
    }

    pub fn user_id(&self) -> &str {
        &self.user_id
    }
}
