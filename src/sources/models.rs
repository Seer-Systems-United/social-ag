#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Post {
    pub id: String,
    pub publisher_user: User,
    pub title: Option<String>,
    pub content: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub url: String,
    pub community: Option<Community>,
    pub in_reply_to_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: String,
    pub username: String,
    pub display_name: Option<String>,
    pub profile_url: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Community {
    pub id: String,
    pub name: String,
    pub display_name: Option<String>,
    pub url: String,
}
