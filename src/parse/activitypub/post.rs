use reqwest::Url;
use serde_json::Value;

use crate::{
    SourceError,
    parse::common::{optional_text, parse_datetime, text_from_html},
    sources::{Post, User},
};

use super::{
    Parser,
    value::{is_public, value_string, value_url},
};

impl Parser {
    pub(super) fn post_from_activity(
        &self,
        activity: &Value,
        user: &User,
    ) -> Result<Option<Post>, SourceError> {
        let object = if value_string(activity, "type") == Some("Create") {
            activity.get("object")
        } else {
            Some(activity)
        };
        let Some(object) = object else {
            return Ok(None);
        };
        let owned;
        let object = if let Some(url) = object.as_str() {
            let url = Url::parse(url).map_err(|_| SourceError::InvalidUrl(url.into()))?;
            owned = self.fetch_activity_value(url)?;
            &owned
        } else {
            object
        };
        if !supported_object(object) || !(is_public(object) || is_public(activity)) {
            return Ok(None);
        }
        let published = value_string(object, "published")
            .or_else(|| value_string(activity, "published"))
            .ok_or_else(|| SourceError::InvalidResponse("missing published date".into()))?;
        let id = value_string(object, "id")
            .ok_or_else(|| SourceError::InvalidResponse("missing object id".into()))?;
        let title = value_string(object, "summary")
            .or_else(|| value_string(object, "name"))
            .and_then(optional_text);
        let content = value_string(object, "content")
            .and_then(text_from_html)
            .or_else(|| value_string(object, "name").and_then(optional_text));
        Ok(Some(Post {
            id: id.into(),
            publisher_user: user.clone(),
            title,
            content,
            timestamp: parse_datetime(published)?,
            url: value_url(object, "url").unwrap_or(id).into(),
            community: None,
            in_reply_to_id: value_string(object, "inReplyTo").map(str::to_string),
        }))
    }
}

fn supported_object(object: &Value) -> bool {
    matches!(
        value_string(object, "type"),
        Some(
            "Note"
                | "Article"
                | "Page"
                | "Image"
                | "Audio"
                | "Video"
                | "Event"
                | "Review"
                | "Comment"
                | "Quotation"
        )
    )
}
