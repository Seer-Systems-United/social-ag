use reqwest::Url;
use scraper::{Html, Selector};

use crate::{TransportConfig, parse::feed, sources::User};

use super::Parser;

impl Parser {
    pub(super) fn fetch_profile_feed(&self, user: &User, count: usize) -> Option<Vec<crate::Post>> {
        let profile_url = Url::parse(&user.profile_url).ok()?;
        let html = self
            .transport
            .get_text(profile_url.clone(), "text/html")
            .ok()?;
        let document = Html::parse_document(&html);
        let selector = Selector::parse("link[rel~=\"alternate\"]").unwrap();
        let href = document.select(&selector).find_map(|link| {
            let value = link.value();
            is_feed_type(value.attr("type"))
                .then(|| value.attr("href"))
                .flatten()
        })?;
        let feed_url = profile_url.join(href).ok()?;
        feed::Parser::new(feed_url.as_str(), user.clone(), TransportConfig::default())
            .ok()?
            .fetch_last_posts(count)
            .ok()
    }
}

fn is_feed_type(media_type: Option<&str>) -> bool {
    matches!(
        media_type,
        Some(
            "application/rss+xml"
                | "application/atom+xml"
                | "application/feed+json"
                | "application/json"
        )
    )
}
