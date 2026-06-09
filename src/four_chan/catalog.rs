use chrono::{TimeZone, Utc};

use crate::{
    Post, SourceError,
    parse::common::{optional_text, text_from_html},
};

use super::{FourChan, models::CatalogPage};

impl FourChan {
    pub(super) fn lookup(&self, identifier: &str) -> Result<Option<crate::User>, SourceError> {
        if self.board != identifier.trim_matches('/') {
            return Ok(None);
        }
        self.fetch_catalog()?;
        Ok(Some(self.user()))
    }

    pub(super) fn fetch_posts(
        &self,
        identifier: &str,
        count: usize,
    ) -> Result<Vec<Post>, SourceError> {
        if count == 0 {
            return Ok(Vec::new());
        }
        if self.board != identifier.trim_matches('/') {
            return Err(SourceError::NotFound);
        }
        let user = self.user();
        let mut posts = self
            .fetch_catalog()?
            .into_iter()
            .flat_map(|page| page.threads)
            .filter_map(|thread| {
                let timestamp = Utc.timestamp_opt(thread.time, 0).single()?;
                Some(Post {
                    id: thread.no.to_string(),
                    publisher_user: user.clone(),
                    title: thread.sub.and_then(optional_text),
                    content: thread.com.as_deref().and_then(text_from_html),
                    timestamp,
                    url: format!(
                        "https://boards.4chan.org/{}/thread/{}",
                        self.board, thread.no
                    ),
                    community: None,
                    in_reply_to_id: None,
                })
            })
            .collect::<Vec<_>>();
        posts.sort_unstable_by_key(|post| std::cmp::Reverse(post.timestamp));
        posts.truncate(count);
        Ok(posts)
    }

    fn fetch_catalog(&self) -> Result<Vec<CatalogPage>, SourceError> {
        self.transport.get_json(self.catalog_url())
    }
}
