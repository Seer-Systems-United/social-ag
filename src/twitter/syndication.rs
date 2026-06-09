use std::cmp::Reverse;

use chrono::{DateTime, Utc};
use scraper::{Html, Selector};

use crate::{
    Post, SourceError, User,
    parse::common::{optional_text, required_identifier},
};

use super::{
    Twitter,
    entities::decode_html_entities,
    syndication_models::{SyndicatedPost, SyndicationData, Timeline},
};

impl Twitter {
    pub(super) fn lookup_syndicated_user(
        &self,
        username: &str,
    ) -> Result<Option<User>, SourceError> {
        let timeline = self.fetch_syndicated_timeline(username)?;
        Ok(timeline.and_then(|timeline| {
            timeline
                .entries
                .into_iter()
                .find_map(|entry| entry.content.tweet)
                .map(|post| syndicated_user(&post))
        }))
    }

    pub(super) fn fetch_syndicated_posts(
        &self,
        username: &str,
        count: usize,
    ) -> Result<Vec<Post>, SourceError> {
        let Some(timeline) = self.fetch_syndicated_timeline(username)? else {
            return Err(SourceError::NotFound);
        };
        let mut posts = timeline
            .entries
            .into_iter()
            .filter_map(|entry| entry.content.tweet)
            .map(convert_post)
            .collect::<Result<Vec<_>, _>>()?;
        posts.sort_unstable_by_key(|post| Reverse(post.timestamp));
        posts.truncate(count);
        Ok(posts)
    }

    fn fetch_syndicated_timeline(&self, username: &str) -> Result<Option<Timeline>, SourceError> {
        let username = required_identifier(username)?.trim_start_matches('@');
        let url = self.syndication_endpoint(username);
        let body = self.transport.get_text(url, "text/html")?;
        let document = Html::parse_document(&body);
        let selector = Selector::parse("script#__NEXT_DATA__").unwrap();
        let json = document
            .select(&selector)
            .next()
            .map(|script| script.text().collect::<String>())
            .ok_or_else(|| SourceError::InvalidResponse("missing X timeline data".into()))?;
        let data: SyndicationData = serde_json::from_str(&json)?;
        Ok(data
            .props
            .page_props
            .context_provider
            .has_results
            .then_some(data.props.page_props.timeline))
    }
}

fn syndicated_user(post: &SyndicatedPost) -> User {
    User {
        id: post.user.screen_name.clone(),
        username: post.user.screen_name.clone(),
        display_name: optional_text(&post.user.name),
        profile_url: format!("https://x.com/{}", post.user.screen_name),
    }
}

fn convert_post(post: SyndicatedPost) -> Result<Post, SourceError> {
    let timestamp = DateTime::parse_from_str(&post.created_at, "%a %b %d %H:%M:%S %z %Y")
        .map_err(|error| SourceError::InvalidResponse(error.to_string()))?
        .with_timezone(&Utc);
    let publisher_user = syndicated_user(&post);
    let text = post.full_text.or(post.text).unwrap_or_default();
    let content = optional_text(decode_html_entities(&text));
    Ok(Post {
        id: post.id_str,
        publisher_user,
        title: None,
        content,
        timestamp,
        url: format!("https://x.com{}", post.permalink),
        community: None,
        in_reply_to_id: post.in_reply_to_status_id_str,
    })
}
