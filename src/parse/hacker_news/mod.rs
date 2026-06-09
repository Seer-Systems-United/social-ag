use reqwest::Url;

use crate::{
    SourceError, TransportConfig,
    parse::common::normalize_base_url,
    sources::{Post, User},
    transport::Transport,
};

#[derive(Clone)]
pub(crate) struct Parser {
    base_url: Url,
    transport: Transport,
}

impl Parser {
    pub(crate) fn new(config: TransportConfig) -> Result<Self, SourceError> {
        Ok(Self {
            base_url: normalize_base_url("https://hacker-news.firebaseio.com/")?,
            transport: Transport::new(config)?,
        })
    }

    pub(crate) fn base_url(&self) -> &Url {
        &self.base_url
    }

    pub(crate) fn set_base_url(&mut self, url: Url) {
        self.base_url = url;
    }

    pub(crate) fn lookup_user_by_username(
        &self,
        username: &str,
    ) -> Result<Option<User>, SourceError> {
        let url = self
            .base_url
            .join(&format!("v0/user/{username}.json"))
            .map_err(|_| SourceError::InvalidUrl(username.to_string()))?;

        #[derive(serde::Deserialize)]
        struct HnUser {
            id: String,
        }

        let hn_user: HnUser = match self.transport.get_json(url) {
            Ok(user) => user,
            Err(SourceError::NotFound) => return Ok(None),
            Err(error) => return Err(error),
        };

        let id = hn_user.id;
        let profile_url = format!("https://news.ycombinator.com/user?id={id}");
        Ok(Some(User {
            id: id.clone(),
            username: id,
            display_name: None,
            profile_url,
        }))
    }

    pub(crate) fn lookup_user_by_id(&self, id: &str) -> Result<Option<User>, SourceError> {
        self.lookup_user_by_username(id)
    }

    pub(crate) fn fetch_last_posts_by_user(
        &self,
        username: &str,
        count: usize,
    ) -> Result<Vec<Post>, SourceError> {
        if count == 0 {
            return Ok(Vec::new());
        }
        let url = self.base_url.join(&format!("v0/user/{username}.json"))
            .map_err(|_| SourceError::InvalidUrl(username.to_string()))?;
        #[derive(serde::Deserialize)]
        struct HnUser { submitted: Option<Vec<u64>> }
        let submitted = match self.transport.get_json::<HnUser>(url)?.submitted {
            Some(ids) => ids,
            None => return Ok(Vec::new()),
        };
        let mut posts = Vec::new();
        let profile_url = format!("https://news.ycombinator.com/user?id={username}");
        for item_id in submitted.iter().take(count) {
            let item_url = self.base_url.join(&format!("v0/item/{item_id}.json"))
                .map_err(|_| SourceError::InvalidUrl(item_id.to_string()))?;
            #[derive(serde::Deserialize)]
            struct HnItem {
                id: u64,
                #[serde(rename = "type")]
                item_type: Option<String>,
                title: Option<String>,
                text: Option<String>,
                url: Option<String>,
                time: Option<i64>,
            }
            let item: HnItem = self.transport.get_json(item_url)?;
            if item.item_type.as_deref() != Some("story") && item.item_type.as_deref() != Some("poll") {
                continue;
            }
            let item_url = item.url.unwrap_or_else(|| format!("https://news.ycombinator.com/item?id={}", item.id));
            let timestamp = item.time.and_then(|t| chrono::DateTime::from_timestamp(t, 0))
                .map(|dt| dt.with_timezone(&chrono::Utc)).unwrap_or_else(chrono::Utc::now);
            posts.push(Post {
                id: item.id.to_string(),
                publisher_user: User {
                    id: username.to_string(),
                    username: username.to_string(),
                    display_name: None,
                    profile_url: profile_url.clone(),
                },
                title: item.title,
                content: item.text,
                timestamp,
                url: item_url,
                community: None,
                in_reply_to_id: None,
            });
        }
        Ok(posts)
    }

    pub(crate) fn fetch_latest_post_by_user(
        &self,
        username: &str,
    ) -> Result<Option<Post>, SourceError> {
        self.fetch_last_posts_by_user(username, 1)
            .map(|posts| posts.into_iter().next())
    }
}

impl std::fmt::Debug for Parser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HackerNewsParser").field("base_url", &self.base_url).finish_non_exhaustive()
    }
}
