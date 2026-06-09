use crate::{Post, SocialSource, SourceError, User};

use super::{CAPABILITIES, Twitter};

impl SocialSource for Twitter {
    fn definition(&self) -> crate::SourceDefinition {
        crate::SourceDefinition {
            name: "Twitter",
            base_url: self.syndication_url.clone(),
            protocol: crate::ParseType::Twitter,
            authentication: crate::Authentication::OptionalBearer,
            capabilities: CAPABILITIES,
            quirks: &[crate::SourceQuirk::UndocumentedPublicEndpoint],
        }
    }

    fn try_lookup_user_by_id(&self, id: &str) -> Result<Option<User>, SourceError> {
        self.lookup_user_by_id(id)
    }

    fn try_lookup_user_by_username(&self, username: &str) -> Result<Option<User>, SourceError> {
        self.lookup_user_by_username(username)
    }

    fn try_lookup_user_by_display_name(&self, _: &str) -> Result<Option<User>, SourceError> {
        Ok(None)
    }

    fn try_fetch_latest_post_by_user(&self, id: &str) -> Result<Option<Post>, SourceError> {
        Ok(self.fetch_posts(id, 1)?.into_iter().next())
    }

    fn try_fetch_last_posts_by_user(
        &self,
        id: &str,
        count: usize,
    ) -> Result<Vec<Post>, SourceError> {
        self.fetch_posts(id, count)
    }
}
