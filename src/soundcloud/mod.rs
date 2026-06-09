use reqwest::Url;
use scraper::{Html, Selector};

use crate::sources::feed::profile_feed_source;

profile_feed_source!(
    SoundCloud,
    "https://soundcloud.com/{username}",
    soundcloud_feed
);

fn soundcloud_feed(document: &Html, _: &Url) -> Option<Url> {
    let selector = Selector::parse("link[href^='android-app://com.soundcloud.android/']").unwrap();
    let href = document.select(&selector).next()?.value().attr("href")?;
    let id = href.rsplit("users:").next()?.parse::<u64>().ok()?;
    format!("https://feeds.soundcloud.com/users/soundcloud:users:{id}/sounds.rss")
        .parse()
        .ok()
}
