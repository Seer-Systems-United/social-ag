use atrium_api::app::bsky::feed::get_author_feed;

use crate::{SourceError, sources::Post};

use super::{Parser, identifier::parse_actor_identifier};

const MAX_FEED_PAGE_SIZE: usize = 100;

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
        let profile = self.lookup_profile(user_id)?.ok_or(SourceError::NotFound)?;
        let actor = parse_actor_identifier(&profile.id)?;
        let expected_id = profile.id;
        let mut cursor: Option<String> = None;
        let mut posts = Vec::with_capacity(count);

        while posts.len() < count {
            let mut url = self.xrpc_url(get_author_feed::NSID);
            let mut query = url.query_pairs_mut();
            query
                .append_pair("actor", actor.as_ref())
                .append_pair("filter", "posts_no_replies")
                .append_pair(
                    "limit",
                    &(count - posts.len()).min(MAX_FEED_PAGE_SIZE).to_string(),
                );
            if let Some(cursor) = &cursor {
                query.append_pair("cursor", cursor);
            }
            drop(query);

            let response: get_author_feed::Output = self.transport.get_json(url)?;
            let atrium_api::types::Object { data: response, .. } = response;
            for item in response.feed {
                let atrium_api::types::Object { data: item, .. } = item;
                if item.post.author.did.as_str() == expected_id {
                    posts.push(self.post_from_view(item.post)?);
                }
                if posts.len() == count {
                    return Ok(posts);
                }
            }
            let Some(next) = response.cursor else {
                break;
            };
            if cursor.as_ref() == Some(&next) {
                break;
            }
            cursor = Some(next);
        }
        Ok(posts)
    }
}
