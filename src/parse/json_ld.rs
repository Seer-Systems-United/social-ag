use scraper::{Html, Selector};
use serde_json::Value;

pub(crate) fn documents(document: &Html) -> Vec<Value> {
    let selector = Selector::parse("script[type='application/ld+json']").unwrap();
    document
        .select(&selector)
        .filter_map(|script| serde_json::from_str(&script.text().collect::<String>()).ok())
        .collect()
}
