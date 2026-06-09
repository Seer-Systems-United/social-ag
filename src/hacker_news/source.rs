use crate::{Capability, Post, SocialSource, SourceError, User};

use super::HackerNews;

impl SocialSource for HackerNews {
    fn definition(&self) -> crate::SourceDefinition {
        crate::SourceDefinition {
            name: "HackerNews",
            base_url: self.parser.base_url.clone(),
            protocol: crate::ParseType::Feed,
            authentication: crate::Authentication::None,
            capabilities: &[
                Capability::LookupUserById,
                Capability::LookupUserByUsername,
                Capability::FetchUserPosts,
            ],
            quirks: &[],
        }
    }

    fn try_lookup_user_by_id(&self, id: &str) -> Result<Option<User>, SourceError> {
        self.parser.lookup_user(id)
    }

    fn try_lookup_user_by_username(&self, username: &str) -> Result<Option<User>, SourceError> {
        self.parser.lookup_user(username)
    }

    fn try_lookup_user_by_display_name(&self, _: &str) -> Result<Option<User>, SourceError> {
        Ok(None)
    }

    fn try_fetch_latest_post_by_user(&self, id: &str) -> Result<Option<Post>, SourceError> {
        Ok(self.parser.fetch_posts(id, 1)?.into_iter().next())
    }

    fn try_fetch_last_posts_by_user(
        &self,
        id: &str,
        count: usize,
    ) -> Result<Vec<Post>, SourceError> {
        self.parser.fetch_posts(id, count)
    }
}
