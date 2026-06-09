use std::io::Cursor;

use atom_syndication::Feed;
use chrono::{DateTime, Utc};
use rss::Channel;

use crate::{SourceError, sources::Post};

use super::Parser;

impl Parser {
    pub(super) fn parse_body(&self, body: &str, count: usize) -> Result<Vec<Post>, SourceError> {
        if let Ok(channel) = Channel::read_from(Cursor::new(body.as_bytes())) {
            return channel
                .items()
                .iter()
                .filter_map(|item| self.post_from_rss(item).transpose())
                .take(count)
                .collect();
        }
        if let Ok(feed) = Feed::read_from(Cursor::new(body.as_bytes())) {
            return feed
                .entries()
                .iter()
                .filter_map(|entry| self.post_from_atom(entry).transpose())
                .take(count)
                .collect();
        }
        self.posts_from_json(body, count)
    }
}

pub(super) fn parse_feed_date(value: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc2822(value)
        .or_else(|_| DateTime::parse_from_rfc3339(value))
        .ok()
        .map(|date| date.with_timezone(&Utc))
}
