mod atom;
mod json;
mod parse;
mod rss;

use reqwest::Url;

use crate::{
    SourceError, TransportConfig,
    parse::common::parse_http_url,
    sources::{Post, User},
    transport::Transport,
};

#[derive(Clone)]
pub(crate) struct Parser {
    pub(super) feed_url: Url,
    pub(super) publisher: User,
    transport: Transport,
}

impl Parser {
    pub(crate) fn new(
        feed_url: impl AsRef<str>,
        publisher: User,
        config: TransportConfig,
    ) -> Result<Self, SourceError> {
        Ok(Self {
            feed_url: parse_http_url(feed_url.as_ref())?,
            publisher,
            transport: Transport::new(config)?,
        })
    }

    pub(crate) fn feed_url(&self) -> &Url {
        &self.feed_url
    }

    pub(crate) fn lookup_user_by_id(&self, id: &str) -> Option<User> {
        (self.publisher.id == id).then(|| self.publisher.clone())
    }

    pub(crate) fn lookup_user_by_username(&self, username: &str) -> Option<User> {
        self.publisher
            .username
            .eq_ignore_ascii_case(username.trim_start_matches('@'))
            .then(|| self.publisher.clone())
    }

    pub(crate) fn lookup_user_by_display_name(&self, name: &str) -> Option<User> {
        self.publisher
            .display_name
            .as_deref()
            .is_some_and(|value| value.eq_ignore_ascii_case(name))
            .then(|| self.publisher.clone())
    }

    pub(crate) fn fetch_last_posts(&self, count: usize) -> Result<Vec<Post>, SourceError> {
        if count == 0 {
            return Ok(Vec::new());
        }
        let body = self.transport.get_text(
            self.feed_url.clone(),
            "application/rss+xml, application/atom+xml, text/xml",
        )?;
        self.parse_body(&body, count)
    }
}

impl std::fmt::Debug for Parser {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("FeedParser")
            .field("feed_url", &self.feed_url)
            .field("publisher", &self.publisher)
            .finish_non_exhaustive()
    }
}
