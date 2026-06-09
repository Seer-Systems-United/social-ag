use crate::{ParseType, SourceError};

use super::{Post, SourceDefinition, User};

pub trait SocialSource: Send + Sync {
    fn definition(&self) -> SourceDefinition;
    fn try_lookup_user_by_id(&self, user_id: &str) -> Result<Option<User>, SourceError>;
    fn try_lookup_user_by_username(&self, username: &str) -> Result<Option<User>, SourceError>;
    fn try_lookup_user_by_display_name(
        &self,
        display_name: &str,
    ) -> Result<Option<User>, SourceError>;
    fn try_fetch_latest_post_by_user(&self, user_id: &str) -> Result<Option<Post>, SourceError>;
    fn try_fetch_last_posts_by_user(
        &self,
        user_id: &str,
        count: usize,
    ) -> Result<Vec<Post>, SourceError>;

    fn parse_type(&self) -> ParseType {
        self.definition().protocol
    }

    fn lookup_user_by_id(&self, user_id: &str) -> Option<User> {
        self.try_lookup_user_by_id(user_id).ok().flatten()
    }

    fn lookup_user_by_username(&self, username: &str) -> Option<User> {
        self.try_lookup_user_by_username(username).ok().flatten()
    }

    fn lookup_user_by_display_name(&self, display_name: &str) -> Option<User> {
        self.try_lookup_user_by_display_name(display_name)
            .ok()
            .flatten()
    }

    fn fetch_latest_post_by_user(&self, user_id: &str) -> Option<Post> {
        self.try_fetch_latest_post_by_user(user_id).ok().flatten()
    }

    fn fetch_last_posts_by_user(&self, user_id: &str, count: usize) -> Vec<Post> {
        self.try_fetch_last_posts_by_user(user_id, count)
            .unwrap_or_default()
    }
}
