use crate::{
    SourceError,
    parse::common::{optional_text, text_from_html},
    sources::Post,
};

use super::{Parser, parse::parse_feed_date};

impl Parser {
    pub(super) fn post_from_rss(&self, item: &rss::Item) -> Result<Option<Post>, SourceError> {
        let Some(url) = item.link().and_then(optional_text) else {
            return Ok(None);
        };
        let Some(timestamp) = item.pub_date().and_then(parse_feed_date) else {
            return Ok(None);
        };
        let id = item
            .guid()
            .map(|guid| guid.value().to_string())
            .unwrap_or_else(|| url.clone());
        let content = item
            .content()
            .or_else(|| item.description())
            .and_then(text_from_html);
        Ok(Some(Post {
            id,
            publisher_user: self.publisher.clone(),
            title: item.title().and_then(optional_text),
            content,
            timestamp,
            url,
            community: None,
            in_reply_to_id: None,
        }))
    }
}
