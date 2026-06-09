use reqwest::Url;
use scraper::Html;

use crate::{
    Post, SourceError, TransportConfig, User, parse::common::parse_http_url, transport::Transport,
};

use super::Parser;

#[derive(Clone)]
pub(crate) struct DiscoveryParser {
    profile_url: Url,
    publisher: User,
    config: TransportConfig,
    transport: Transport,
    resolver: fn(&Html, &Url) -> Option<Url>,
}

impl DiscoveryParser {
    pub(crate) fn new(
        profile_url: impl AsRef<str>,
        publisher: User,
        config: TransportConfig,
        resolver: fn(&Html, &Url) -> Option<Url>,
    ) -> Result<Self, SourceError> {
        Ok(Self {
            profile_url: parse_http_url(profile_url.as_ref())?,
            publisher,
            transport: Transport::new(config.clone())?,
            config,
            resolver,
        })
    }

    pub(crate) fn profile_url(&self) -> &Url {
        &self.profile_url
    }

    pub(crate) fn lookup_user_by_id(&self, id: &str) -> Result<Option<User>, SourceError> {
        self.lookup(self.publisher.id == id)
    }
    pub(crate) fn lookup_user_by_username(
        &self,
        username: &str,
    ) -> Result<Option<User>, SourceError> {
        self.lookup(
            self.publisher
                .username
                .eq_ignore_ascii_case(username.trim_start_matches('@')),
        )
    }

    pub(crate) fn lookup_user_by_display_name(&self, _: &str) -> Result<Option<User>, SourceError> {
        Ok(None)
    }

    pub(crate) fn fetch_latest_post_by_user(&self, id: &str) -> Result<Option<Post>, SourceError> {
        Ok(self.fetch_last_posts_by_user(id, 1)?.into_iter().next())
    }

    pub(crate) fn fetch_last_posts_by_user(
        &self,
        id: &str,
        count: usize,
    ) -> Result<Vec<Post>, SourceError> {
        if count == 0 {
            return Ok(Vec::new());
        }
        if self.publisher.id != id && !self.publisher.username.eq_ignore_ascii_case(id) {
            return Err(SourceError::NotFound);
        }
        let feed_url = self.discover_feed()?;
        Parser::new(feed_url, self.publisher.clone(), self.config.clone())?.fetch_last_posts(count)
    }

    fn lookup(&self, matches: bool) -> Result<Option<User>, SourceError> {
        if !matches {
            return Ok(None);
        }
        self.discover_feed()?;
        Ok(Some(self.publisher.clone()))
    }

    fn discover_feed(&self) -> Result<Url, SourceError> {
        let body = self
            .transport
            .get_text(self.profile_url.clone(), "text/html")?;
        (self.resolver)(&Html::parse_document(&body), &self.profile_url)
            .ok_or(SourceError::NotFound)
    }
}
