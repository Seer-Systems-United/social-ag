use chrono::{DateTime, Utc};
use std::collections::HashSet;

use crate::{Post, SourceError, parse::common::optional_text};

use super::{
    Facebook,
    models::{PageList, PagePost},
};

impl Facebook {
    pub(super) fn fetch_posts(
        &self,
        identifier: &str,
        count: usize,
    ) -> Result<Vec<Post>, SourceError> {
        if count == 0 {
            return Ok(Vec::new());
        }
        let publisher = self.lookup_page(identifier)?.ok_or(SourceError::NotFound)?;
        let mut posts = Vec::with_capacity(count);
        let mut after: Option<String> = None;
        let mut visited = HashSet::new();
        while posts.len() < count {
            let mut url = self.endpoint(&[&publisher.id, "published_posts"]);
            let limit = (count - posts.len()).min(100);
            url.query_pairs_mut()
                .append_pair("fields", "id,message,story,created_time,permalink_url")
                .append_pair("limit", &limit.to_string());
            if let Some(cursor) = &after {
                url.query_pairs_mut().append_pair("after", cursor);
            }
            let page: PageList<PagePost> = self.transport.get_json(url)?;
            posts.extend(convert_posts(page.data, &publisher)?);
            let next = page
                .paging
                .and_then(|paging| paging.cursors)
                .and_then(|cursors| cursors.after);
            let Some(next) = next.filter(|cursor| visited.insert(cursor.clone())) else {
                break;
            };
            after = Some(next);
        }
        posts.truncate(count);
        Ok(posts)
    }
}

fn convert_posts(items: Vec<PagePost>, publisher: &crate::User) -> Result<Vec<Post>, SourceError> {
    items
        .into_iter()
        .map(|post| convert_post(post, publisher))
        .collect()
}

fn convert_post(post: PagePost, publisher: &crate::User) -> Result<Post, SourceError> {
    let timestamp = DateTime::parse_from_rfc3339(&post.created_time)
        .or_else(|_| DateTime::parse_from_str(&post.created_time, "%Y-%m-%dT%H:%M:%S%z"))
        .map_err(|error| SourceError::InvalidResponse(error.to_string()))?
        .with_timezone(&Utc);
    Ok(Post {
        url: post
            .permalink_url
            .unwrap_or_else(|| format!("https://www.facebook.com/{}", post.id)),
        id: post.id,
        publisher_user: publisher.clone(),
        title: None,
        content: post.message.or(post.story).and_then(optional_text),
        timestamp,
        community: None,
        in_reply_to_id: None,
    })
}
