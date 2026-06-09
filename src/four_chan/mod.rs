mod catalog;
mod models;
mod source;

use reqwest::Url;

use crate::{
    SourceError, TransportConfig, User, parse::common::normalize_base_url, transport::Transport,
};

const API_URL: &str = "https://a.4cdn.org/";

#[derive(Clone)]
pub struct FourChan {
    api_url: Url,
    board: String,
    transport: Transport,
}

impl FourChan {
    pub fn new(board: impl AsRef<str>) -> Result<Self, SourceError> {
        Self::new_with_config(board, TransportConfig::default())
    }

    pub fn new_with_config(
        board: impl AsRef<str>,
        config: TransportConfig,
    ) -> Result<Self, SourceError> {
        let board = board.as_ref().trim_matches('/').to_string();
        if board.is_empty() {
            return Err(SourceError::InvalidIdentifier(board));
        }
        Ok(Self {
            api_url: Url::parse(API_URL).unwrap(),
            board,
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

    pub(super) fn catalog_url(&self) -> Url {
        let mut url = self.api_url.clone();
        url.path_segments_mut()
            .unwrap()
            .extend([self.board.as_str(), "catalog.json"]);
        url
    }

    pub(super) fn user(&self) -> User {
        User {
            id: self.board.clone(),
            username: self.board.clone(),
            display_name: Some(format!("/{}/", self.board)),
            profile_url: format!("https://boards.4chan.org/{}/", self.board),
        }
    }
}

impl std::fmt::Debug for FourChan {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("FourChan")
            .field("api_url", &self.api_url)
            .field("board", &self.board)
            .finish_non_exhaustive()
    }
}
