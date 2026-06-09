use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct ApiResponse<T> {
    pub data: Option<T>,
    pub meta: Option<Meta>,
}

impl<T> ApiResponse<T> {
    pub fn into_data(self) -> Option<T> {
        self.data
    }
}

#[derive(Debug, Deserialize)]
pub(super) struct Meta {
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(super) struct ApiUser {
    pub id: String,
    pub name: String,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct ApiPost {
    pub id: String,
    pub text: String,
    pub created_at: String,
    #[serde(default)]
    pub referenced_tweets: Vec<ReferencedPost>,
}

#[derive(Debug, Deserialize)]
pub(super) struct ReferencedPost {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
}
