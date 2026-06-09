use chrono::{DateTime, Utc};
use reqwest::Url;
use scraper::{Html, Selector};

use crate::SourceError;

pub(crate) fn normalize_base_url(value: &str) -> Result<Url, SourceError> {
    let mut url = parse_http_url(value)?;

    url.set_query(None);
    url.set_fragment(None);

    if !url.path().ends_with('/') {
        let path = format!("{}/", url.path());
        url.set_path(&path);
    }

    Ok(url)
}

pub(crate) fn parse_http_url(value: &str) -> Result<Url, SourceError> {
    let url = Url::parse(value).map_err(|_| SourceError::InvalidUrl(value.to_string()))?;

    if !matches!(url.scheme(), "http" | "https") || url.host_str().is_none() {
        return Err(SourceError::InvalidUrl(value.to_string()));
    }

    Ok(url)
}

pub(crate) fn required_identifier(value: &str) -> Result<&str, SourceError> {
    let value = value.trim();
    if value.is_empty() {
        Err(SourceError::InvalidIdentifier(value.to_string()))
    } else {
        Ok(value)
    }
}

pub(crate) fn parse_datetime(value: &str) -> Result<DateTime<Utc>, SourceError> {
    DateTime::parse_from_rfc3339(value)
        .map(|date| date.with_timezone(&Utc))
        .map_err(|error| SourceError::InvalidResponse(error.to_string()))
}

pub(crate) fn text_from_html(content: &str) -> Option<String> {
    let fragment = Html::parse_fragment(content);
    let paragraph_selector = Selector::parse("p").unwrap();
    let paragraphs = fragment
        .select(&paragraph_selector)
        .filter_map(|paragraph| optional_text(paragraph.text().collect::<String>()))
        .collect::<Vec<_>>();

    if !paragraphs.is_empty() {
        return Some(paragraphs.join("\n\n"));
    }

    optional_text(fragment.root_element().text().collect::<String>())
}

pub(crate) fn optional_text(value: impl AsRef<str>) -> Option<String> {
    let normalized = value
        .as_ref()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    (!normalized.is_empty()).then_some(normalized)
}
