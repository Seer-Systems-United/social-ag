use crate::{SourceError, sources::User};

use super::{Parser, models};

impl Parser {
    pub(super) fn lookup_user(&self, username: &str) -> Result<Option<User>, SourceError> {
        let url = self
            .base_url
            .join(&format!("v0/user/{username}.json"))
            .map_err(|_| SourceError::InvalidUrl(username.to_string()))?;
        let user: models::User = match self.transport.get_json(url) {
            Ok(user) => user,
            Err(SourceError::NotFound) => return Ok(None),
            Err(error) => return Err(error),
        };
        let id = user.id;
        Ok(Some(User {
            profile_url: format!("https://news.ycombinator.com/user?id={id}"),
            id: id.clone(),
            username: id,
            display_name: None,
        }))
    }
}
