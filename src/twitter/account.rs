use crate::{SourceError, parse::common::required_identifier, sources::User};

use super::{
    Twitter,
    models::{ApiResponse, ApiUser},
};

impl Twitter {
    pub(super) fn lookup_user_by_id(&self, id: &str) -> Result<Option<User>, SourceError> {
        let id = required_identifier(id)?;
        if self.authenticated {
            self.fetch_api_user(self.api_endpoint(&["2", "users", id]))
        } else {
            self.lookup_syndicated_user(id)
        }
    }

    pub(super) fn lookup_user_by_username(
        &self,
        username: &str,
    ) -> Result<Option<User>, SourceError> {
        let username = required_identifier(username)?.trim_start_matches('@');
        if self.authenticated {
            self.fetch_api_user(self.api_endpoint(&["2", "users", "by", "username", username]))
        } else {
            self.lookup_syndicated_user(username)
        }
    }

    pub(super) fn resolve_user(&self, identifier: &str) -> Result<User, SourceError> {
        let identifier = required_identifier(identifier)?;
        let user = if identifier
            .chars()
            .all(|character| character.is_ascii_digit())
        {
            self.lookup_user_by_id(identifier)?
        } else {
            self.lookup_user_by_username(identifier)?
        };
        user.ok_or(SourceError::NotFound)
    }

    fn fetch_api_user(&self, mut url: reqwest::Url) -> Result<Option<User>, SourceError> {
        url.query_pairs_mut()
            .append_pair("user.fields", "name,username");
        let response: ApiResponse<ApiUser> = match self.transport.get_json(url) {
            Ok(response) => response,
            Err(SourceError::NotFound) => return Ok(None),
            Err(error) => return Err(error),
        };
        Ok(response.into_data().map(User::from))
    }
}

impl From<ApiUser> for User {
    fn from(user: ApiUser) -> Self {
        Self {
            id: user.id,
            profile_url: format!("https://x.com/{}", user.username),
            username: user.username,
            display_name: Some(user.name),
        }
    }
}
