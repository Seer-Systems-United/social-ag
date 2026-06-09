use serde::Deserialize;

use crate::{
    SourceError,
    parse::common::{optional_text, parse_datetime, text_from_html},
    sources::{Post, User},
};

#[derive(Debug, Deserialize)]
pub(super) struct ApiInstance {
    pub(super) version: String,
    pub(super) api_versions: Option<ApiVersions>,
}

#[derive(Debug, Deserialize)]
pub(super) struct ApiVersions {
    pub(super) mastodon: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub(super) struct ApiAccount {
    pub(super) id: String,
    pub(super) username: String,
    #[serde(default)]
    pub(super) display_name: String,
    pub(super) url: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct ApiStatus {
    pub(super) id: String,
    pub(super) created_at: String,
    #[serde(default)]
    pub(super) spoiler_text: String,
    #[serde(default)]
    pub(super) content: String,
    pub(super) account: ApiAccount,
    pub(super) url: Option<String>,
    pub(super) uri: String,
    pub(super) in_reply_to_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(super) struct ApiStatusAccount {
    pub(super) account: ApiAccount,
}

impl From<ApiAccount> for User {
    fn from(account: ApiAccount) -> Self {
        Self {
            id: account.id,
            username: account.username,
            display_name: optional_text(account.display_name),
            profile_url: account.url,
        }
    }
}

impl TryFrom<ApiStatus> for Post {
    type Error = SourceError;

    fn try_from(status: ApiStatus) -> Result<Self, Self::Error> {
        Ok(Self {
            id: status.id,
            publisher_user: status.account.into(),
            title: optional_text(status.spoiler_text),
            content: text_from_html(&status.content),
            timestamp: parse_datetime(&status.created_at)?,
            url: status.url.unwrap_or(status.uri),
            community: None,
            in_reply_to_id: status.in_reply_to_id,
        })
    }
}
