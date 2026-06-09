use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct Page {
    pub id: String,
    pub name: String,
    pub link: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(super) struct PagePost {
    pub id: String,
    pub message: Option<String>,
    pub story: Option<String>,
    pub created_time: String,
    pub permalink_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(super) struct PageList<T> {
    pub data: Vec<T>,
    pub paging: Option<Paging>,
}

#[derive(Debug, Deserialize)]
pub(super) struct Paging {
    pub cursors: Option<Cursors>,
}

#[derive(Debug, Deserialize)]
pub(super) struct Cursors {
    pub after: Option<String>,
}
