use crate::{SourceError, sources::Post};

use super::{Parser, models::ApiStatus};

const MAX_STATUSES_PER_REQUEST: usize = 40;

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
        let account_id = self.resolve_account_id(user_id)?;
        let mut posts = Vec::with_capacity(count);
        let mut max_id: Option<String> = None;

        while posts.len() < count {
            let requested = (count - posts.len()).min(MAX_STATUSES_PER_REQUEST);
            let mut url = self.api_url(&["accounts", &account_id, "statuses"]);
            let mut query = url.query_pairs_mut();
            query
                .append_pair("limit", &requested.to_string())
                .append_pair("exclude_reblogs", "true");
            if let Some(max_id) = &max_id {
                query.append_pair("max_id", max_id);
            }
            drop(query);

            let statuses: Vec<ApiStatus> = self.transport.get_json(url)?;
            let Some(next_id) = statuses.last().map(|status| status.id.clone()) else {
                break;
            };
            if max_id.as_ref() == Some(&next_id) {
                break;
            }
            for status in statuses {
                posts.push(Post::try_from(status)?);
                if posts.len() == count {
                    return Ok(posts);
                }
            }
            max_id = Some(next_id);
        }
        Ok(posts)
    }
}
