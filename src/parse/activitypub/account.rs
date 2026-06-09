use reqwest::Url;

use crate::{SourceError, parse::common::required_identifier, sources::User};

use super::{
    ACTIVITY_ACCEPT, Parser,
    models::{Actor, WebFinger},
};

impl Parser {
    pub(crate) fn lookup_user_by_id(&self, user_id: &str) -> Result<Option<User>, SourceError> {
        let identifier = required_identifier(user_id)?;
        let Ok(url) = Url::parse(identifier) else {
            return self.lookup_user_by_username(identifier);
        };
        match self.fetch_actor(url) {
            Ok(actor) => Ok(Some(actor.into())),
            Err(SourceError::NotFound) => Ok(None),
            Err(error) => Err(error),
        }
    }

    pub(crate) fn lookup_user_by_username(
        &self,
        username: &str,
    ) -> Result<Option<User>, SourceError> {
        let Some(actor_url) = self.webfinger_actor_url(username)? else {
            return Ok(None);
        };
        self.lookup_user_by_id(actor_url.as_str())
    }

    pub(crate) fn lookup_user_by_display_name(&self, _: &str) -> Result<Option<User>, SourceError> {
        Err(SourceError::Unsupported {
            capability: "display-name search",
        })
    }

    pub(super) fn resolve_actor(&self, identifier: &str) -> Result<Actor, SourceError> {
        let identifier = required_identifier(identifier)?;
        if let Ok(url) = Url::parse(identifier) {
            return self.fetch_actor(url);
        }
        let url = self
            .webfinger_actor_url(identifier)?
            .ok_or(SourceError::NotFound)?;
        self.fetch_actor(url)
    }

    fn fetch_actor(&self, url: Url) -> Result<Actor, SourceError> {
        let body = self.transport.get_text(url, ACTIVITY_ACCEPT)?;
        serde_json::from_str(&body).map_err(SourceError::from)
    }

    fn webfinger_actor_url(&self, username: &str) -> Result<Option<Url>, SourceError> {
        let username = required_identifier(username)?.trim_start_matches('@');
        let (username, domain) = username.rsplit_once('@').unwrap_or((
            username,
            self.instance_url
                .host_str()
                .ok_or_else(|| SourceError::InvalidUrl(self.instance_url.to_string()))?,
        ));
        let mut url = Url::parse(&format!("https://{domain}/.well-known/webfinger"))
            .map_err(|_| SourceError::InvalidUrl(domain.into()))?;
        url.query_pairs_mut()
            .append_pair("resource", &format!("acct:{username}@{domain}"));
        let response: WebFinger = match self.transport.get_json(url) {
            Ok(response) => response,
            Err(SourceError::NotFound) => return Ok(None),
            Err(error) => return Err(error),
        };
        response.actor_url()
    }
}
