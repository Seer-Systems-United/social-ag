use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Deserialize;

use crate::SourceError;

#[derive(Clone)]
pub(super) struct CatalogItem {
    pub id: String,
    pub url: reqwest::Url,
}

#[derive(Deserialize)]
pub(super) struct Release {
    #[serde(rename = "@id")]
    pub url: String,
    pub name: String,
    #[serde(rename = "datePublished")]
    pub published: String,
    pub description: Option<String>,
}

impl Release {
    pub fn timestamp(&self) -> Result<DateTime<Utc>, SourceError> {
        NaiveDateTime::parse_from_str(&self.published, "%d %b %Y %H:%M:%S GMT")
            .map(|value| value.and_utc())
            .map_err(|error| SourceError::InvalidResponse(error.to_string()))
    }
}
