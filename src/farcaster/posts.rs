use chrono::{TimeZone, Utc};

use crate::{Post, SourceError, parse::common::optional_text};

use super::{
    Farcaster,
    models::{CastMessage, CastPage},
};

const FARCASTER_EPOCH: i64 = 1_609_459_200;

impl Farcaster {
    pub(super) fn posts(&self, identifier: &str, count: usize) -> Result<Vec<Post>, SourceError> {
        if count == 0 {
            return Ok(Vec::new());
        }
        let Some(user) = self.lookup(identifier)? else {
            return Err(SourceError::NotFound);
        };
        let mut token = None;
        let mut posts = Vec::with_capacity(count);
        loop {
            let page: CastPage = self
                .transport
                .get_json(self.casts_page(count, token.as_deref()))?;
            posts.extend(
                page.messages
                    .into_iter()
                    .filter_map(|message| post_from_cast(message, &user).transpose())
                    .collect::<Result<Vec<_>, _>>()?,
            );
            if posts.len() >= count {
                break;
            }
            let Some(next) = page.next_page_token else {
                break;
            };
            token = Some(next);
        }
        posts.truncate(count);
        Ok(posts)
    }

    fn casts_page(&self, count: usize, token: Option<&str>) -> reqwest::Url {
        let mut url = self.endpoint("castsByFid");
        let mut query = url.query_pairs_mut();
        query
            .append_pair("pageSize", &count.min(100).to_string())
            .append_pair("reverse", "true");
        if let Some(token) = token {
            query.append_pair("pageToken", token);
        }
        drop(query);
        url
    }
}

fn post_from_cast(message: CastMessage, user: &crate::User) -> Result<Option<Post>, SourceError> {
    let Some(content) = optional_text(message.data.body.text) else {
        return Ok(None);
    };
    let timestamp = Utc
        .timestamp_opt(FARCASTER_EPOCH + message.data.timestamp, 0)
        .single()
        .ok_or_else(|| SourceError::InvalidResponse("invalid Farcaster timestamp".into()))?;
    Ok(Some(Post {
        id: message.hash.clone(),
        publisher_user: user.clone(),
        title: None,
        content: Some(content),
        timestamp,
        url: format!("https://farcaster.xyz/v/{}", message.hash),
        community: None,
        in_reply_to_id: message.data.body.parent.map(|parent| parent.hash),
    }))
}
