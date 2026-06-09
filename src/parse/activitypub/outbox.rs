use std::collections::HashSet;

use reqwest::Url;
use serde_json::Value;

use crate::{
    SourceError,
    sources::{Post, User},
};

use super::{
    ACTIVITY_ACCEPT, Parser,
    value::{collection_items, collection_link},
};

impl Parser {
    pub(crate) fn fetch_latest_post_by_user(
        &self,
        user_id: &str,
    ) -> Result<Option<Post>, SourceError> {
        Ok(self
            .fetch_last_posts_by_user(user_id, 1)?
            .into_iter()
            .next())
    }

    pub(crate) fn fetch_last_posts_by_user(
        &self,
        user_id: &str,
        count: usize,
    ) -> Result<Vec<Post>, SourceError> {
        if count == 0 {
            return Ok(Vec::new());
        }
        let actor = self.resolve_actor(user_id)?;
        if actor.indexable == Some(false) {
            return Err(SourceError::Unsupported {
                capability: "indexing posts from this actor",
            });
        }
        let user = User::from(actor.clone());
        let outbox = actor.outbox.ok_or(SourceError::Unsupported {
            capability: "actor outbox",
        })?;
        let url = outbox
            .parse()
            .map_err(|_| SourceError::InvalidUrl(outbox))?;
        let posts = self.fetch_outbox(url, user.clone(), count)?;
        if posts.is_empty() {
            return Ok(self.fetch_profile_feed(&user, count).unwrap_or(posts));
        }
        Ok(posts)
    }

    fn fetch_outbox(&self, url: Url, user: User, count: usize) -> Result<Vec<Post>, SourceError> {
        let mut visited = HashSet::from([url.clone()]);
        let mut collection = self.fetch_activity_value(url)?;
        let mut posts = Vec::with_capacity(count);
        loop {
            if let Some(items) = collection_items(&collection) {
                for item in items {
                    if let Some(post) = self.post_from_activity(item, &user)? {
                        posts.push(post);
                        if posts.len() == count {
                            return Ok(posts);
                        }
                    }
                }
            }
            let next = collection_link(&collection, "next")
                .or_else(|| collection_link(&collection, "first").filter(|_| posts.is_empty()));
            let Some(next) = next else {
                return Ok(posts);
            };
            if !visited.insert(next.clone()) {
                return Err(SourceError::InvalidResponse("pagination loop".into()));
            }
            collection = self.fetch_activity_value(next)?;
        }
    }

    pub(super) fn fetch_activity_value(&self, url: Url) -> Result<Value, SourceError> {
        let body = self.transport.get_text(url, ACTIVITY_ACCEPT)?;
        serde_json::from_str(&body).map_err(SourceError::from)
    }
}
