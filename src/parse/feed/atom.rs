use chrono::Utc;

use crate::{
    SourceError,
    parse::common::{optional_text, text_from_html},
    sources::Post,
};

use super::Parser;

impl Parser {
    pub(super) fn post_from_atom(
        &self,
        entry: &atom_syndication::Entry,
    ) -> Result<Option<Post>, SourceError> {
        let Some(url) = entry
            .links()
            .iter()
            .find(|link| link.rel() == "alternate")
            .or_else(|| entry.links().first())
            .map(|link| link.href().to_string())
        else {
            return Ok(None);
        };
        let timestamp = entry
            .published()
            .copied()
            .unwrap_or_else(|| *entry.updated())
            .with_timezone(&Utc);
        let content = entry
            .content()
            .and_then(|content| content.value())
            .or_else(|| entry.summary().map(|summary| summary.as_str()))
            .and_then(text_from_html);
        Ok(Some(Post {
            id: entry.id().into(),
            publisher_user: self.publisher.clone(),
            title: optional_text(entry.title().as_str()),
            content,
            timestamp,
            url,
            community: None,
            in_reply_to_id: None,
        }))
    }
}
