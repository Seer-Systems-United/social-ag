use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct CatalogPage {
    #[serde(default)]
    pub threads: Vec<CatalogThread>,
}

#[derive(Debug, Deserialize)]
pub(super) struct CatalogThread {
    pub no: u64,
    pub time: i64,
    pub sub: Option<String>,
    pub com: Option<String>,
}
