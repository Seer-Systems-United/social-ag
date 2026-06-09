use std::{fmt, time::Duration};

use chrono::{DateTime, Utc};
use reqwest::{
    Url,
    blocking::{Client, RequestBuilder},
};
use scraper::{Html, Selector};
use serde::Deserialize;

use crate::sources::{Post, User};

const MAX_STATUSES_PER_REQUEST: usize = 40;
const REQUEST_TIMEOUT: Duration = Duration::from_secs(15);
const USER_AGENT: &str = "Mozilla/5.0 (compatible; social-ag/0.1)";

#[derive(Clone)]
pub(crate) struct Parser {
    instance_url: Url,
    client: Client,
    access_token: Option<String>,
}

impl Parser {
    pub(crate) fn new(instance_url: impl AsRef<str>) -> Result<Self, MastodonError> {
        let instance_url = normalize_instance_url(instance_url.as_ref())?;
        let client = Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .user_agent(USER_AGENT)
            .build()
            .map_err(MastodonError::Request)?;

        Ok(Self {
            instance_url,
            client,
            access_token: None,
        })
    }

    pub(crate) fn with_access_token(mut self, access_token: impl Into<String>) -> Self {
        let access_token = access_token.into();
        self.access_token = (!access_token.trim().is_empty()).then_some(access_token);
        self
    }

    pub(crate) fn instance_url(&self) -> &Url {
        &self.instance_url
    }

    pub(crate) fn lookup_user_by_id(&self, user_id: &str) -> Result<Option<User>, MastodonError> {
        let user_id = required_identifier(user_id)?;
        let url = self.api_url(&["accounts", user_id]);
        let response = self.request(url).send().map_err(MastodonError::Request)?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }

        if matches!(
            response.status(),
            reqwest::StatusCode::UNAUTHORIZED | reqwest::StatusCode::FORBIDDEN
        ) {
            return self.lookup_user_from_statuses(user_id);
        }

        response
            .error_for_status()
            .map_err(MastodonError::Request)?
            .json::<ApiAccount>()
            .map(User::from)
            .map(Some)
            .map_err(MastodonError::Request)
    }

    pub(crate) fn lookup_user_by_username(
        &self,
        username: &str,
    ) -> Result<Option<User>, MastodonError> {
        let username = required_identifier(username)?.trim_start_matches('@');
        let mut url = self.api_url(&["accounts", "lookup"]);
        url.query_pairs_mut().append_pair("acct", username);
        self.fetch_optional_account(url)
    }

    pub(crate) fn lookup_user_by_display_name(
        &self,
        display_name: &str,
    ) -> Result<Option<User>, MastodonError> {
        let display_name = required_identifier(display_name)?;
        let mut url = self.api_url(&["accounts", "search"]);
        url.query_pairs_mut()
            .append_pair("q", display_name)
            .append_pair("limit", "40");

        let accounts: Vec<ApiAccount> = self.fetch_json(url)?;
        Ok(accounts
            .into_iter()
            .find(|account| account.display_name.eq_ignore_ascii_case(display_name))
            .map(User::from))
    }

    pub(crate) fn fetch_latest_post_by_user(
        &self,
        user_id: &str,
    ) -> Result<Option<Post>, MastodonError> {
        Ok(self
            .fetch_last_posts_by_user(user_id, 1)?
            .into_iter()
            .next())
    }

    pub(crate) fn fetch_last_posts_by_user(
        &self,
        user_id: &str,
        count: usize,
    ) -> Result<Vec<Post>, MastodonError> {
        if count == 0 {
            return Ok(Vec::new());
        }

        let account_id = self.resolve_account_id(user_id)?;
        let mut posts = Vec::with_capacity(count);
        let mut max_id: Option<String> = None;

        while posts.len() < count {
            let requested = (count - posts.len()).min(MAX_STATUSES_PER_REQUEST);
            let mut url = self.api_url(&["accounts", &account_id, "statuses"]);

            {
                let mut query = url.query_pairs_mut();
                query
                    .append_pair("limit", &requested.to_string())
                    .append_pair("exclude_reblogs", "true");

                if let Some(max_id) = &max_id {
                    query.append_pair("max_id", max_id);
                }
            }

            let statuses: Vec<ApiStatus> = self.fetch_json(url)?;
            let Some(next_max_id) = statuses.last().map(|status| status.id.clone()) else {
                break;
            };

            if max_id.as_ref() == Some(&next_max_id) {
                break;
            }

            for status in statuses {
                posts.push(Post::try_from(status)?);
                if posts.len() == count {
                    break;
                }
            }

            max_id = Some(next_max_id);
        }

        Ok(posts)
    }

    fn resolve_account_id(&self, user_id: &str) -> Result<String, MastodonError> {
        let identifier = required_identifier(user_id)?;

        if identifier
            .chars()
            .all(|character| character.is_ascii_digit())
        {
            return Ok(identifier.to_string());
        }

        self.lookup_user_by_username(identifier)?
            .map(|user| user.id)
            .ok_or_else(|| MastodonError::UserNotFound(identifier.to_string()))
    }

    fn fetch_optional_account(&self, url: Url) -> Result<Option<User>, MastodonError> {
        let response = self.request(url).send().map_err(MastodonError::Request)?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }

        response
            .error_for_status()
            .map_err(MastodonError::Request)?
            .json::<ApiAccount>()
            .map(User::from)
            .map(Some)
            .map_err(MastodonError::Request)
    }

    fn lookup_user_from_statuses(&self, user_id: &str) -> Result<Option<User>, MastodonError> {
        let mut url = self.api_url(&["accounts", user_id, "statuses"]);
        url.query_pairs_mut().append_pair("limit", "1");

        let statuses: Vec<ApiStatusAccount> = self.fetch_json(url)?;
        Ok(statuses
            .into_iter()
            .next()
            .map(|status| status.account.into()))
    }

    fn fetch_json<T>(&self, url: Url) -> Result<T, MastodonError>
    where
        T: serde::de::DeserializeOwned,
    {
        self.request(url)
            .send()
            .map_err(MastodonError::Request)?
            .error_for_status()
            .map_err(MastodonError::Request)?
            .json()
            .map_err(MastodonError::Request)
    }

    fn api_url(&self, path: &[&str]) -> Url {
        let mut url = self.instance_url.join("api/v1/").unwrap();
        {
            let mut segments = url.path_segments_mut().unwrap();
            segments.pop_if_empty();
            segments.extend(path);
        }
        url
    }

    fn request(&self, url: Url) -> RequestBuilder {
        let request = self
            .client
            .get(url)
            .header(reqwest::header::ACCEPT, "application/json");

        match &self.access_token {
            Some(access_token) => request.bearer_auth(access_token),
            None => request,
        }
    }
}

impl fmt::Debug for Parser {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MastodonParser")
            .field("instance_url", &self.instance_url)
            .field("has_access_token", &self.access_token.is_some())
            .finish()
    }
}

#[derive(Debug)]
pub enum MastodonError {
    EmptyUserIdentifier,
    InvalidInstanceUrl(String),
    InvalidTimestamp {
        value: String,
        source: chrono::ParseError,
    },
    Request(reqwest::Error),
    UserNotFound(String),
}

impl fmt::Display for MastodonError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyUserIdentifier => formatter.write_str("user identifier cannot be empty"),
            Self::InvalidInstanceUrl(url) => {
                write!(formatter, "invalid Mastodon instance URL: {url}")
            }
            Self::InvalidTimestamp { value, source } => {
                write!(formatter, "invalid status timestamp {value:?}: {source}")
            }
            Self::Request(error) => write!(formatter, "Mastodon API request failed: {error}"),
            Self::UserNotFound(identifier) => {
                write!(formatter, "Mastodon user not found: {identifier}")
            }
        }
    }
}

impl std::error::Error for MastodonError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidTimestamp { source, .. } => Some(source),
            Self::Request(error) => Some(error),
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize)]
struct ApiAccount {
    id: String,
    username: String,
    #[serde(default)]
    display_name: String,
    url: String,
}

#[derive(Debug, Deserialize)]
struct ApiStatus {
    id: String,
    created_at: String,
    #[serde(default)]
    spoiler_text: String,
    #[serde(default)]
    content: String,
    account: ApiAccount,
    url: Option<String>,
    uri: String,
}

#[derive(Debug, Deserialize)]
struct ApiStatusAccount {
    account: ApiAccount,
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
    type Error = MastodonError;

    fn try_from(status: ApiStatus) -> Result<Self, Self::Error> {
        let timestamp = DateTime::parse_from_rfc3339(&status.created_at)
            .map_err(|source| MastodonError::InvalidTimestamp {
                value: status.created_at,
                source,
            })?
            .with_timezone(&Utc);

        Ok(Self {
            id: status.id,
            publisher_user: status.account.into(),
            title: optional_text(status.spoiler_text),
            content: text_from_html(&status.content),
            timestamp,
            url: status.url.unwrap_or(status.uri),
        })
    }
}

fn required_identifier(value: &str) -> Result<&str, MastodonError> {
    let value = value.trim();
    if value.is_empty() {
        Err(MastodonError::EmptyUserIdentifier)
    } else {
        Ok(value)
    }
}

fn normalize_instance_url(instance_url: &str) -> Result<Url, MastodonError> {
    let mut url = Url::parse(instance_url)
        .map_err(|_| MastodonError::InvalidInstanceUrl(instance_url.to_string()))?;

    if !matches!(url.scheme(), "http" | "https") || url.host_str().is_none() {
        return Err(MastodonError::InvalidInstanceUrl(instance_url.to_string()));
    }

    url.set_query(None);
    url.set_fragment(None);

    if !url.path().ends_with('/') {
        let path = format!("{}/", url.path());
        url.set_path(&path);
    }

    Ok(url)
}

fn text_from_html(content: &str) -> Option<String> {
    let fragment = Html::parse_fragment(content);
    let paragraph_selector = Selector::parse("p").unwrap();
    let paragraphs = fragment
        .select(&paragraph_selector)
        .filter_map(|paragraph| optional_text(paragraph.text().collect::<String>()))
        .collect::<Vec<_>>();

    if !paragraphs.is_empty() {
        return Some(paragraphs.join("\n\n"));
    }

    optional_text(fragment.root_element().text().collect::<String>())
}

fn optional_text(value: impl AsRef<str>) -> Option<String> {
    let normalized = value
        .as_ref()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    (!normalized.is_empty()).then_some(normalized)
}

#[cfg(test)]
mod tests {
    use super::{ApiStatus, MastodonError, Parser, text_from_html};
    use crate::Post;

    #[test]
    fn normalizes_instance_and_builds_encoded_api_urls() {
        let parser = Parser::new("https://example.social/subpath?ignored=yes").unwrap();
        let url = parser.api_url(&["accounts", "user/id", "statuses"]);

        assert_eq!(
            url.as_str(),
            "https://example.social/subpath/api/v1/accounts/user%2Fid/statuses"
        );
    }

    #[test]
    fn rejects_non_http_instance_urls() {
        assert!(matches!(
            Parser::new("file:///tmp/mastodon"),
            Err(MastodonError::InvalidInstanceUrl(_))
        ));
    }

    #[test]
    fn converts_status_json_to_public_models() {
        let status: ApiStatus = serde_json::from_str(
            r#"{
                "id": "123",
                "created_at": "2026-06-08T20:15:30.000Z",
                "spoiler_text": "Release notes",
                "content": "<p>Hello <strong>Fediverse</strong>.</p><p>Second paragraph.</p>",
                "url": "https://example.social/@mastodon/123",
                "uri": "https://example.social/users/mastodon/statuses/123",
                "account": {
                    "id": "42",
                    "username": "mastodon",
                    "display_name": "Mastodon",
                    "url": "https://example.social/@mastodon"
                }
            }"#,
        )
        .unwrap();

        let post = Post::try_from(status).unwrap();

        assert_eq!(post.id, "123");
        assert_eq!(post.publisher_user.id, "42");
        assert_eq!(post.publisher_user.username, "mastodon");
        assert_eq!(
            post.publisher_user.profile_url,
            "https://example.social/@mastodon"
        );
        assert_eq!(post.title.as_deref(), Some("Release notes"));
        assert_eq!(
            post.content.as_deref(),
            Some("Hello Fediverse.\n\nSecond paragraph.")
        );
        assert_eq!(post.timestamp.to_rfc3339(), "2026-06-08T20:15:30+00:00");
        assert_eq!(post.url, "https://example.social/@mastodon/123");
    }

    #[test]
    fn extracts_complete_text_from_nested_mastodon_markup() {
        let content = text_from_html(
            r##"<p><a href="#"><span class="invisible">https://</span><span>example.com</span></a> <span>@<span>user</span></span></p>"##,
        );

        assert_eq!(content.as_deref(), Some("https://example.com @user"));
    }

    #[test]
    fn empty_html_content_is_none() {
        assert_eq!(text_from_html("<p></p>"), None);
    }
}
