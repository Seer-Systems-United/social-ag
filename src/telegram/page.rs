use std::cmp::Reverse;

use chrono::{DateTime, Utc};
use scraper::{Html, Selector};

use crate::{Post, SourceError, User, parse::common::text_from_html};

use super::Telegram;

impl Telegram {
    pub(super) fn lookup(&self, identifier: &str) -> Result<Option<User>, SourceError> {
        if self.user.id != identifier
            && !self
                .user
                .username
                .eq_ignore_ascii_case(identifier.trim_start_matches('@'))
        {
            return Ok(None);
        }
        let document = self.fetch_document()?;
        Ok(Some(user_from_document(&document, &self.user)))
    }

    pub(super) fn fetch_posts(
        &self,
        identifier: &str,
        count: usize,
    ) -> Result<Vec<Post>, SourceError> {
        if count == 0 {
            return Ok(Vec::new());
        }
        let Some(_) = self.lookup(identifier)? else {
            return Err(SourceError::NotFound);
        };
        let document = self.fetch_document()?;
        let user = user_from_document(&document, &self.user);
        let message_selector = Selector::parse(".tgme_widget_message[data-post]").unwrap();
        let mut posts = document
            .select(&message_selector)
            .filter_map(|message| post_from_element(message, &user).transpose())
            .collect::<Result<Vec<_>, _>>()?;
        posts.sort_unstable_by_key(|post| Reverse(post.timestamp));
        posts.truncate(count);
        Ok(posts)
    }

    fn fetch_document(&self) -> Result<Html, SourceError> {
        let body = self
            .transport
            .get_text(self.fetch_url.clone(), "text/html")?;
        Ok(Html::parse_document(&body))
    }
}

fn user_from_document(document: &Html, fallback: &User) -> User {
    let selector = Selector::parse(".tgme_channel_info_header_title").unwrap();
    let mut user = fallback.clone();
    user.display_name = document.select(&selector).next().and_then(|element| {
        crate::parse::common::optional_text(element.text().collect::<String>())
    });
    user
}

fn post_from_element(
    message: scraper::ElementRef<'_>,
    user: &User,
) -> Result<Option<Post>, SourceError> {
    let data_post = message.value().attr("data-post").unwrap_or_default();
    let id = data_post.rsplit('/').next().unwrap_or_default();
    let time_selector = Selector::parse("time[datetime]").unwrap();
    let Some(datetime) = message
        .select(&time_selector)
        .next()
        .and_then(|time| time.value().attr("datetime"))
    else {
        return Ok(None);
    };
    let timestamp = DateTime::parse_from_rfc3339(datetime)
        .map_err(|error| SourceError::InvalidResponse(error.to_string()))?
        .with_timezone(&Utc);
    let text_selector = Selector::parse(".tgme_widget_message_text").unwrap();
    let content = message
        .select(&text_selector)
        .next()
        .and_then(|text| text_from_html(&text.inner_html()));
    Ok(Some(Post {
        id: id.into(),
        publisher_user: user.clone(),
        title: None,
        content,
        timestamp,
        url: format!("https://t.me/{data_post}"),
        community: None,
        in_reply_to_id: None,
    }))
}
