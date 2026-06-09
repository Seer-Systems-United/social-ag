use serde_json::json;

use crate::{SourceError, User, parse::common::optional_text};

use super::{
    Lens,
    models::{AccountData, ApiAccount},
    queries,
};

impl Lens {
    pub(super) fn lookup(&self, identifier: &str) -> Result<Option<User>, SourceError> {
        let Some(account) = self.account()? else {
            return Ok(None);
        };
        let user = user_from_account(account)?;
        let identifier = identifier.trim_start_matches('@');
        Ok((user.id.eq_ignore_ascii_case(identifier)
            || user.username.eq_ignore_ascii_case(identifier)
            || user
                .username
                .eq_ignore_ascii_case(identifier.strip_prefix("lens/").unwrap_or(identifier))
            || user
                .display_name
                .as_deref()
                .is_some_and(|name| name.eq_ignore_ascii_case(identifier)))
        .then_some(user))
    }

    pub(super) fn account(&self) -> Result<Option<ApiAccount>, SourceError> {
        let data: AccountData = self.query(queries::ACCOUNT, json!({"username": self.username}))?;
        Ok(data.account)
    }
}

pub(super) fn user_from_account(account: ApiAccount) -> Result<User, SourceError> {
    let username = account
        .username
        .ok_or_else(|| SourceError::InvalidResponse("missing Lens username".into()))?;
    Ok(User {
        id: account.address,
        username: username.local_name.clone(),
        display_name: account
            .metadata
            .and_then(|metadata| metadata.name)
            .and_then(optional_text),
        profile_url: format!("https://hey.xyz/u/{}", username.local_name),
    })
}
