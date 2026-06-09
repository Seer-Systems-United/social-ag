use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct GraphResponse<T> {
    pub data: Option<T>,
    #[serde(default)]
    pub errors: Vec<GraphError>,
}

#[derive(Deserialize)]
pub(super) struct GraphError {
    pub message: String,
}

#[derive(Deserialize)]
pub(super) struct AccountData {
    pub account: Option<ApiAccount>,
}

#[derive(Clone, Deserialize)]
pub(super) struct ApiAccount {
    pub address: String,
    pub username: Option<ApiUsername>,
    pub metadata: Option<AccountMetadata>,
}

#[derive(Clone, Deserialize)]
pub(super) struct ApiUsername {
    #[serde(rename = "localName")]
    pub local_name: String,
}

#[derive(Clone, Deserialize)]
pub(super) struct AccountMetadata {
    pub name: Option<String>,
}

#[derive(Deserialize)]
pub(super) struct PostsData {
    pub posts: PostPage,
}

#[derive(Deserialize)]
pub(super) struct PostPage {
    #[serde(default)]
    pub items: Vec<ApiPost>,
    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
}

#[derive(Deserialize)]
pub(super) struct PageInfo {
    pub next: Option<String>,
}

#[derive(Deserialize)]
pub(super) struct ApiPost {
    pub id: String,
    pub slug: String,
    pub timestamp: String,
    pub metadata: PostMetadata,
}

#[derive(Deserialize)]
pub(super) struct PostMetadata {
    pub title: Option<String>,
    pub content: Option<String>,
}
