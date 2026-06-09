use crate::parse::ParseType;

pub(crate) mod mastodon;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Post {
    pub id: String,
    pub publisher_user: User,
    pub title: Option<String>,
    pub content: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: String,
    pub username: String,
    pub display_name: Option<String>,
    pub profile_url: String,
}

pub trait SocialSource {
    fn parse_type(&self) -> ParseType;

    fn lookup_user_by_id(&self, user_id: &str) -> Option<User>;
    fn lookup_user_by_username(&self, username: &str) -> Option<User>;
    fn lookup_user_by_display_name(&self, display_name: &str) -> Option<User>;

    fn fetch_latest_post_by_user(&self, user_id: &str) -> Option<Post>;
    fn fetch_last_posts_by_user(&self, user_id: &str, count: usize) -> Vec<Post>;
}
