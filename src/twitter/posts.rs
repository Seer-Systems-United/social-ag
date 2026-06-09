use std::collections::HashSet;

use crate::{
    Post, SourceError,
    parse::common::{optional_text, parse_datetime},
};

use super::{
    Twitter,
    models::{ApiPost, ApiResponse},
};

impl Twitter {
    pub(super) fn fetch_posts(
        &self,
        identifier: &str,
        count: usize,
    ) -> Result<Vec<Post>, SourceError> {
        if count == 0 {
            return Ok(Vec::new());
        }
        if !self.authenticated {
            return self.fetch_syndicated_posts(identifier, count);
        }
        self.fetch_api_posts(identifier, count)
    }

    fn fetch_api_posts(&self, identifier: &str, count: usize) -> Result<Vec<Post>, SourceError> {
        let publisher = self.resolve_user(identifier)?;
        let mut posts = Vec::with_capacity(count);
        let mut pagination_token: Option<String> = None;
        let mut visited = HashSet::new();
        while posts.len() < count {
            let mut url = self.api_endpoint(&["2", "users", &publisher.id, "tweets"]);
            let requested = (count - posts.len()).clamp(5, 100);
            url.query_pairs_mut()
                .append_pair("max_results", &requested.to_string())
                .append_pair("tweet.fields", "created_at,referenced_tweets");
            if let Some(token) = &pagination_token {
                url.query_pairs_mut().append_pair("pagination_token", token);
            }
            let response: ApiResponse<Vec<ApiPost>> = self.transport.get_json(url)?;
            let converted = response
                .data
                .unwrap_or_default()
                .into_iter()
                .map(|post| convert_post(post, &publisher))
                .collect::<Result<Vec<_>, _>>()?;
            posts.extend(converted);
            let next = response.meta.and_then(|meta| meta.next_token);
            let Some(next) = next.filter(|token| visited.insert(token.clone())) else {
                break;
            };
            pagination_token = Some(next);
        }
        posts.truncate(count);
        Ok(posts)
    }
}

fn convert_post(post: ApiPost, publisher: &crate::User) -> Result<Post, SourceError> {
    let in_reply_to_id = post
        .referenced_tweets
        .iter()
        .find(|reference| reference.kind == "replied_to")
        .map(|reference| reference.id.clone());
    Ok(Post {
        id: post.id.clone(),
        publisher_user: publisher.clone(),
        title: None,
        content: optional_text(post.text),
        timestamp: parse_datetime(&post.created_at)?,
        url: format!("https://x.com/{}/status/{}", publisher.username, post.id),
        community: None,
        in_reply_to_id,
    })
}
