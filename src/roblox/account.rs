use crate::{SourceError, User, parse::common::optional_text};

use super::{Roblox, models::ApiUser};

impl Roblox {
    pub(super) fn lookup(&self, identifier: &str) -> Result<Option<User>, SourceError> {
        let user: User = self.fetch_user()?.into();
        let identifier = identifier.trim_start_matches('@');
        Ok((user.id == identifier
            || user.username.eq_ignore_ascii_case(identifier)
            || user
                .display_name
                .as_deref()
                .is_some_and(|name| name.eq_ignore_ascii_case(identifier)))
        .then_some(user))
    }

    pub(super) fn fetch_user(&self) -> Result<ApiUser, SourceError> {
        let mut url = self.users_url.clone();
        url.path_segments_mut()
            .unwrap()
            .pop_if_empty()
            .push(&self.user_id);
        self.transport.get_json(url)
    }
}

impl From<ApiUser> for User {
    fn from(user: ApiUser) -> Self {
        Self {
            id: user.id.to_string(),
            username: user.name,
            display_name: optional_text(user.display_name),
            profile_url: format!("https://www.roblox.com/users/{}/profile", user.id),
        }
    }
}
