use crate::{
    Authentication, Capability, ParseType, Post, SocialSource, SourceDefinition, SourceError,
    TransportConfig, User, parse::feed::Parser,
};

const CAPABILITIES: &[Capability] = &[
    Capability::LookupUserById,
    Capability::LookupUserByUsername,
    Capability::LookupUserByDisplayName,
    Capability::FetchUserPosts,
];

#[derive(Debug, Clone)]
pub struct FeedSource {
    parser: Parser,
}

impl FeedSource {
    pub fn new(feed_url: impl AsRef<str>, publisher: User) -> Result<Self, SourceError> {
        Self::new_with_config(feed_url, publisher, TransportConfig::default())
    }

    pub fn new_with_config(
        feed_url: impl AsRef<str>,
        publisher: User,
        config: TransportConfig,
    ) -> Result<Self, SourceError> {
        Ok(Self {
            parser: Parser::new(feed_url, publisher, config)?,
        })
    }
}

impl SocialSource for FeedSource {
    fn definition(&self) -> SourceDefinition {
        SourceDefinition {
            name: "FeedSource",
            base_url: self.parser.feed_url().clone(),
            protocol: ParseType::Feed,
            authentication: Authentication::None,
            capabilities: CAPABILITIES,
            quirks: &[],
        }
    }

    fn try_lookup_user_by_id(&self, user_id: &str) -> Result<Option<User>, SourceError> {
        Ok(self.parser.lookup_user_by_id(user_id))
    }

    fn try_lookup_user_by_username(&self, username: &str) -> Result<Option<User>, SourceError> {
        Ok(self.parser.lookup_user_by_username(username))
    }

    fn try_lookup_user_by_display_name(
        &self,
        display_name: &str,
    ) -> Result<Option<User>, SourceError> {
        Ok(self.parser.lookup_user_by_display_name(display_name))
    }

    fn try_fetch_latest_post_by_user(&self, user_id: &str) -> Result<Option<Post>, SourceError> {
        Ok(self
            .try_fetch_last_posts_by_user(user_id, 1)?
            .into_iter()
            .next())
    }

    fn try_fetch_last_posts_by_user(
        &self,
        user_id: &str,
        count: usize,
    ) -> Result<Vec<Post>, SourceError> {
        if self.parser.lookup_user_by_id(user_id).is_none()
            && self.parser.lookup_user_by_username(user_id).is_none()
        {
            return Err(SourceError::NotFound);
        }

        self.parser.fetch_last_posts(count)
    }
}
