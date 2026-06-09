use serde_json::json;

use crate::{
    Post, SourceError,
    parse::common::{optional_text, parse_datetime},
};

use super::{
    Lens,
    account::user_from_account,
    models::{ApiPost, PostsData},
    queries,
};

impl Lens {
    pub(super) fn posts(&self, identifier: &str, count: usize) -> Result<Vec<Post>, SourceError> {
        if count == 0 {
            return Ok(Vec::new());
        }
        let Some(account) = self.account()? else {
            return Err(SourceError::NotFound);
        };
        let user = user_from_account(account)?;
        if user.id != identifier
            && !user
                .username
                .eq_ignore_ascii_case(identifier.trim_start_matches('@'))
        {
            return Err(SourceError::NotFound);
        }
        let mut cursor = None;
        let mut posts = Vec::with_capacity(count);
        while posts.len() < count {
            let page_size = if count - posts.len() <= 10 {
                "TEN"
            } else {
                "FIFTY"
            };
            let data: PostsData = self.query(
                queries::POSTS,
                json!({"address": user.id, "pageSize": page_size, "cursor": cursor}),
            )?;
            posts.extend(
                data.posts
                    .items
                    .into_iter()
                    .map(|post| post_from_api(post, &user))
                    .collect::<Result<Vec<_>, _>>()?,
            );
            let Some(next) = data.posts.page_info.next else {
                break;
            };
            cursor = Some(next);
        }
        posts.truncate(count);
        Ok(posts)
    }
}

fn post_from_api(post: ApiPost, user: &crate::User) -> Result<Post, SourceError> {
    Ok(Post {
        id: post.id,
        publisher_user: user.clone(),
        title: post.metadata.title.and_then(optional_text),
        content: post.metadata.content.and_then(optional_text),
        timestamp: parse_datetime(&post.timestamp)?,
        url: format!("https://hey.xyz/posts/{}", post.slug),
        community: None,
        in_reply_to_id: None,
    })
}
