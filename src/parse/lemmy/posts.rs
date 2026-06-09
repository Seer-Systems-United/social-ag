use crate::{SourceError, sources::Post};

use super::{Parser, models::PersonDetails};

const MAX_PAGE_SIZE: usize = 50;

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
        let person = self.resolve_person(user_id)?;
        let mut posts = Vec::with_capacity(count);
        let mut page = 1;

        while posts.len() < count {
            let requested = (count - posts.len()).min(MAX_PAGE_SIZE);
            let mut url = self.api_url("user");
            url.query_pairs_mut()
                .append_pair("person_id", &person.id)
                .append_pair("sort", "New")
                .append_pair("page", &page.to_string())
                .append_pair("limit", &requested.to_string());
            let response: PersonDetails = self.transport.get_json(url)?;
            let page_len = response.posts.len();
            for view in response.posts {
                posts.push(Post::try_from(view)?);
                if posts.len() == count {
                    return Ok(posts);
                }
            }
            if page_len < requested {
                break;
            }
            page += 1;
        }
        Ok(posts)
    }
}
