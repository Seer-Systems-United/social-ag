use reqwest::Url;
use scraper::{Html, Selector};

pub(crate) fn alternate_feed(document: &Html, profile_url: &Url) -> Option<Url> {
    let selector = Selector::parse("link[rel~='alternate'][href]").unwrap();
    document.select(&selector).find_map(|link| {
        let kind = link.value().attr("type")?.to_ascii_lowercase();
        (kind.contains("rss") || kind.contains("atom") || kind.contains("feed"))
            .then(|| profile_url.join(link.value().attr("href")?).ok())
            .flatten()
    })
}
