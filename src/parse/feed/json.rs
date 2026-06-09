use serde::Deserialize;

use crate::{
    SourceError,
    parse::common::{optional_text, text_from_html},
    sources::Post,
};

use super::{Parser, parse::parse_feed_date};

#[derive(Deserialize)]
struct JsonFeed {
    items: Vec<JsonItem>,
}

#[derive(Deserialize)]
struct JsonItem {
    id: String,
    url: Option<String>,
    external_url: Option<String>,
    title: Option<String>,
    content_html: Option<String>,
    content_text: Option<String>,
    date_published: Option<String>,
    date_modified: Option<String>,
}

impl Parser {
    pub(super) fn posts_from_json(
        &self,
        body: &str,
        count: usize,
    ) -> Result<Vec<Post>, SourceError> {
        let feed: JsonFeed = serde_json::from_str(body)?;
        Ok(feed
            .items
            .into_iter()
            .filter_map(|item| self.post_from_json(item))
            .take(count)
            .collect())
    }

    fn post_from_json(&self, item: JsonItem) -> Option<Post> {
        let url = item.url.or(item.external_url)?;
        let timestamp = item
            .date_published
            .or(item.date_modified)
            .as_deref()
            .and_then(parse_feed_date)?;
        let content = item
            .content_html
            .as_deref()
            .and_then(text_from_html)
            .or_else(|| item.content_text.and_then(optional_text));
        Some(Post {
            id: item.id,
            publisher_user: self.publisher.clone(),
            title: item.title.and_then(optional_text),
            content,
            timestamp,
            url,
            community: None,
            in_reply_to_id: None,
        })
    }
}
