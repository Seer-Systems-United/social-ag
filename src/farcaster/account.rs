use crate::{SourceError, User, parse::common::optional_text};

use super::{Farcaster, models::ProfilePage};

impl Farcaster {
    pub(super) fn lookup(&self, identifier: &str) -> Result<Option<User>, SourceError> {
        let user = self.profile()?;
        let identifier = identifier.trim_start_matches('@');
        Ok((user.id == identifier
            || user.username.eq_ignore_ascii_case(identifier)
            || user
                .display_name
                .as_deref()
                .is_some_and(|name| name.eq_ignore_ascii_case(identifier)))
        .then_some(user))
    }

    pub(super) fn profile(&self) -> Result<User, SourceError> {
        let page: ProfilePage = self.transport.get_json(self.endpoint("userDataByFid"))?;
        let fid = page
            .messages
            .first()
            .map(|message| message.data.fid)
            .ok_or(SourceError::NotFound)?;
        let value = |kind: &str| {
            page.messages
                .iter()
                .find(|message| message.data.body.kind == kind)
                .map(|message| message.data.body.value.clone())
        };
        let username = value("USER_DATA_TYPE_USERNAME")
            .ok_or_else(|| SourceError::InvalidResponse("missing Farcaster username".into()))?;
        Ok(User {
            id: fid.to_string(),
            display_name: value("USER_DATA_TYPE_DISPLAY").and_then(optional_text),
            profile_url: format!("https://farcaster.xyz/@{username}"),
            username,
        })
    }
}
