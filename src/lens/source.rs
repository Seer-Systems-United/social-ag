use crate::{Authentication, Capability, ParseType, Post, SocialSource, SourceError, User};

use super::Lens;

const CAPABILITIES: &[Capability] = &[
    Capability::LookupUserById,
    Capability::LookupUserByUsername,
    Capability::LookupUserByDisplayName,
    Capability::FetchUserPosts,
];

impl SocialSource for Lens {
    fn definition(&self) -> crate::SourceDefinition {
        crate::SourceDefinition {
            name: "Lens",
            base_url: self.api_url.clone(),
            protocol: ParseType::GraphQl,
            authentication: Authentication::None,
            capabilities: CAPABILITIES,
            quirks: &[],
        }
    }

    fn try_lookup_user_by_id(&self, id: &str) -> Result<Option<User>, SourceError> {
        self.lookup(id)
    }

    fn try_lookup_user_by_username(&self, username: &str) -> Result<Option<User>, SourceError> {
        self.lookup(username)
    }

    fn try_lookup_user_by_display_name(&self, name: &str) -> Result<Option<User>, SourceError> {
        self.lookup(name)
    }

    fn try_fetch_latest_post_by_user(&self, id: &str) -> Result<Option<Post>, SourceError> {
        Ok(self.posts(id, 1)?.into_iter().next())
    }

    fn try_fetch_last_posts_by_user(
        &self,
        id: &str,
        count: usize,
    ) -> Result<Vec<Post>, SourceError> {
        self.posts(id, count)
    }
}
