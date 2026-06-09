use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct User {
    pub id: String,
    pub submitted: Option<Vec<u64>>,
}

#[derive(Deserialize)]
pub(super) struct Item {
    pub id: u64,
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    pub title: Option<String>,
    pub text: Option<String>,
    pub url: Option<String>,
    pub time: Option<i64>,
}
