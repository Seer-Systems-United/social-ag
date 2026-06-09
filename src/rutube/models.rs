use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct VideoPage {
    pub next: Option<String>,
    #[serde(default)]
    pub results: Vec<Video>,
}

#[derive(Debug, Deserialize)]
pub(super) struct Video {
    pub id: String,
    pub title: String,
    pub description: String,
    pub publication_ts: String,
    pub video_url: String,
    pub author: Author,
}

#[derive(Debug, Deserialize)]
pub(super) struct Author {
    pub id: u64,
    pub name: String,
}
