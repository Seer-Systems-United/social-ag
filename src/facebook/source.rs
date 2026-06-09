use crate::{Post, SocialSource, SourceError, User};

use super::{CAPABILITIES, Facebook};

impl SocialSource for Facebook {
    fn definition(&self) -> crate::SourceDefinition {
        crate::SourceDefinition {
            name: "Facebook",
            base_url: self.api_url.clone(),
            protocol: crate::ParseType::Facebook,
            authentication: crate::Authentication::RequiredBearer,
            capabilities: CAPABILITIES,
            quirks: &[],
        }
    }

    fn try_lookup_user_by_id(&self, id: &str) -> Result<Option<User>, SourceError> {
        self.lookup_page(id)
    }

    fn try_lookup_user_by_username(&self, username: &str) -> Result<Option<User>, SourceError> {
        self.lookup_page(username.trim_start_matches('@'))
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
