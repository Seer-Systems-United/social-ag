use reqwest::Url;

use crate::{
    Authentication, Capability, ParseType, SourceDefinition, SourceError, SocialSource,
    TransportConfig, sources::{Post, User},
};

#[derive(Debug, Clone)]
pub struct HackerNews {
    parser: crate::parse::hacker_news::Parser,
}

impl HackerNews {
    pub fn new() -> Result<Self, SourceError> {
        Self::new_with_config(TransportConfig::default())
    }

    pub fn new_with_config(config: TransportConfig) -> Result<Self, SourceError> {
        Ok(Self {
            parser: crate::parse::hacker_news::Parser::new(config)?,
        })
    }

    pub fn with_base_url(mut self, base_url: impl AsRef<str>) -> Result<Self, SourceError> {
        self.parser.set_base_url(Url::parse(base_url.as_ref())
            .map_err(|_| SourceError::InvalidUrl(base_url.as_ref().to_string()))?);
        Ok(self)
    }

    pub fn base_url(&self) -> &reqwest::Url {
        self.parser.base_url()
    }
}

impl Default for HackerNews {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

impl SocialSource for HackerNews {
    fn definition(&self) -> SourceDefinition {
        SourceDefinition {
            name: "HackerNews",
            base_url: self.parser.base_url().clone(),
            protocol: ParseType::Feed,
            authentication: Authentication::None,
            capabilities: &[
                Capability::LookupUserById,
                Capability::LookupUserByUsername,
                Capability::FetchUserPosts,
            ],
            quirks: &[],
        }
    }

    fn try_lookup_user_by_id(&self, id: &str) -> Result<Option<User>, SourceError> {
        self.parser.lookup_user_by_id(id)
    }

    fn try_lookup_user_by_username(&self, username: &str) -> Result<Option<User>, SourceError> {
        self.parser.lookup_user_by_username(username)
    }

    fn try_lookup_user_by_display_name(
        &self,
        _name: &str,
    ) -> Result<Option<User>, SourceError> {
        Ok(None)
    }

    fn try_fetch_latest_post_by_user(&self, user_id: &str) -> Result<Option<Post>, SourceError> {
        self.parser.fetch_latest_post_by_user(user_id)
    }

    fn try_fetch_last_posts_by_user(
        &self,
        user_id: &str,
        count: usize,
    ) -> Result<Vec<Post>, SourceError> {
        self.parser.fetch_last_posts_by_user(user_id, count)
    }
}
