use reqwest::Url;
use serde::Deserialize;
use serde_json::Value;

use crate::{SourceError, parse::common::optional_text, sources::User};

use super::value::value_as_url;

#[derive(Debug, Deserialize)]
pub(super) struct WebFinger {
    #[serde(default)]
    links: Vec<WebFingerLink>,
}

#[derive(Debug, Deserialize)]
struct WebFingerLink {
    rel: String,
    #[serde(rename = "type")]
    media_type: Option<String>,
    href: Option<String>,
}

impl WebFinger {
    pub(super) fn actor_url(self) -> Result<Option<Url>, SourceError> {
        self.links
            .into_iter()
            .find(|link| {
                link.rel == "self"
                    && link
                        .media_type
                        .as_deref()
                        .is_none_or(|value| value.contains("activity"))
            })
            .and_then(|link| link.href)
            .map(|href| {
                Url::parse(&href)
                    .map_err(|_| SourceError::InvalidResponse("invalid actor URL".into()))
            })
            .transpose()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct Actor {
    pub(super) id: String,
    #[serde(rename = "preferredUsername")]
    preferred_username: String,
    name: Option<String>,
    url: Option<Value>,
    pub(super) outbox: Option<String>,
    pub(super) indexable: Option<bool>,
}

impl From<Actor> for User {
    fn from(actor: Actor) -> Self {
        let profile_url = actor
            .url
            .as_ref()
            .and_then(value_as_url)
            .unwrap_or(&actor.id)
            .to_string();
        Self {
            id: actor.id,
            username: actor.preferred_username,
            display_name: actor.name.and_then(optional_text),
            profile_url,
        }
    }
}
