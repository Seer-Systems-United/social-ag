use crate::{Post, SourceError, sources::User};

use super::{Parser, models};

impl Parser {
    pub(super) fn fetch_posts(
        &self,
        username: &str,
        count: usize,
    ) -> Result<Vec<Post>, SourceError> {
        if count == 0 {
            return Ok(Vec::new());
        }
        let url = self
            .base_url
            .join(&format!("v0/user/{username}.json"))
            .map_err(|_| SourceError::InvalidUrl(username.to_string()))?;
        let submitted = self
            .transport
            .get_json::<models::User>(url)?
            .submitted
            .unwrap_or_default();
        let publisher = User {
            id: username.into(),
            username: username.into(),
            display_name: None,
            profile_url: format!("https://news.ycombinator.com/user?id={username}"),
        };
        let mut posts = Vec::new();
        for item_id in submitted.into_iter().take(count) {
            let url = self
                .base_url
                .join(&format!("v0/item/{item_id}.json"))
                .map_err(|_| SourceError::InvalidUrl(item_id.to_string()))?;
            let item: models::Item = self.transport.get_json(url)?;
            if matches!(item.item_type.as_deref(), Some("story" | "poll")) {
                posts.push(convert_item(item, &publisher));
            }
        }
        Ok(posts)
    }
}

fn convert_item(item: models::Item, publisher: &User) -> Post {
    Post {
        id: item.id.to_string(),
        publisher_user: publisher.clone(),
        title: item.title,
        content: item.text,
        timestamp: item
            .time
            .and_then(|value| chrono::DateTime::from_timestamp(value, 0))
            .unwrap_or_else(chrono::Utc::now),
        url: item
            .url
            .unwrap_or_else(|| format!("https://news.ycombinator.com/item?id={}", item.id)),
        community: None,
        in_reply_to_id: None,
    }
}
