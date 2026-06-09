use chrono::NaiveDateTime;

use crate::{Post, SourceError, User, parse::common::optional_text};

use super::{
    Rutube,
    models::{Author, Video, VideoPage},
};

impl Rutube {
    pub(super) fn lookup(&self, identifier: &str) -> Result<Option<User>, SourceError> {
        if self.channel_id != identifier.trim_matches('/') {
            return Ok(None);
        }
        let page: VideoPage = self.transport.get_json(self.channel_url())?;
        Ok(page
            .results
            .first()
            .map(|video| user_from_author(&video.author)))
    }

    pub(super) fn fetch_posts(
        &self,
        identifier: &str,
        count: usize,
    ) -> Result<Vec<Post>, SourceError> {
        if count == 0 {
            return Ok(Vec::new());
        }
        if self.channel_id != identifier.trim_matches('/') {
            return Err(SourceError::NotFound);
        }
        let mut url = Some(self.channel_url());
        let mut posts = Vec::with_capacity(count);
        while let Some(page_url) = url {
            let page: VideoPage = self.transport.get_json(page_url)?;
            posts.extend(
                page.results
                    .into_iter()
                    .map(post_from_video)
                    .collect::<Result<Vec<_>, _>>()?,
            );
            if posts.len() >= count {
                break;
            }
            url = page.next.and_then(|next| next.parse().ok());
        }
        posts.truncate(count);
        Ok(posts)
    }
}

fn user_from_author(author: &Author) -> User {
    let id = author.id.to_string();
    User {
        id: id.clone(),
        username: id,
        display_name: optional_text(&author.name),
        profile_url: format!("https://rutube.ru/channel/{}/", author.id),
    }
}

fn post_from_video(video: Video) -> Result<Post, SourceError> {
    let timestamp = NaiveDateTime::parse_from_str(&video.publication_ts, "%Y-%m-%dT%H:%M:%S")
        .map_err(|error| SourceError::InvalidResponse(error.to_string()))?
        .and_utc();
    Ok(Post {
        id: video.id,
        publisher_user: user_from_author(&video.author),
        title: optional_text(video.title),
        content: optional_text(video.description),
        timestamp,
        url: video.video_url,
        community: None,
        in_reply_to_id: None,
    })
}
