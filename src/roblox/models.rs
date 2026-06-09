use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct ApiUser {
    pub id: u64,
    pub name: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
}

#[derive(Deserialize)]
pub(super) struct GamePage {
    #[serde(rename = "nextPageCursor")]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub data: Vec<Game>,
}

#[derive(Deserialize)]
pub(super) struct Game {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub updated: String,
    #[serde(rename = "rootPlace")]
    pub root_place: RootPlace,
}

#[derive(Deserialize)]
pub(super) struct RootPlace {
    pub id: u64,
}
