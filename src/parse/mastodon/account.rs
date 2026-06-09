use crate::{SourceError, SourceQuirk, parse::common::required_identifier, sources::User};

use super::{
    Parser,
    models::{ApiAccount, ApiStatusAccount},
};

impl Parser {
    pub(crate) fn lookup_user_by_id(&self, user_id: &str) -> Result<Option<User>, SourceError> {
        let user_id = required_identifier(user_id)?;
        match self
            .transport
            .get_json::<ApiAccount>(self.api_url(&["accounts", user_id]))
        {
            Ok(account) => Ok(Some(account.into())),
            Err(SourceError::NotFound) => Ok(None),
            Err(SourceError::AuthenticationRequired | SourceError::Blocked { .. })
                if self
                    .quirks
                    .contains(&SourceQuirk::AccountByIdRequiresStatusFallback) =>
            {
                self.lookup_user_from_statuses(user_id)
            }
            Err(error) => Err(error),
        }
    }

    pub(crate) fn lookup_user_by_username(
        &self,
        username: &str,
    ) -> Result<Option<User>, SourceError> {
        let username = required_identifier(username)?.trim_start_matches('@');
        let mut url = self.api_url(&["accounts", "lookup"]);
        url.query_pairs_mut().append_pair("acct", username);
        match self.transport.get_json::<ApiAccount>(url) {
            Ok(account) => Ok(Some(account.into())),
            Err(SourceError::NotFound) => Ok(None),
            Err(error) => Err(error),
        }
    }

    pub(crate) fn lookup_user_by_display_name(
        &self,
        display_name: &str,
    ) -> Result<Option<User>, SourceError> {
        let display_name = required_identifier(display_name)?;
        let mut url = self.api_url(&["accounts", "search"]);
        url.query_pairs_mut()
            .append_pair("q", display_name)
            .append_pair("limit", "40");
        let accounts: Vec<ApiAccount> = self.transport.get_json(url)?;
        Ok(accounts
            .into_iter()
            .find(|account| account.display_name.eq_ignore_ascii_case(display_name))
            .map(User::from))
    }

    pub(super) fn resolve_account_id(&self, user_id: &str) -> Result<String, SourceError> {
        let identifier = required_identifier(user_id)?;
        if identifier.chars().all(|value| value.is_ascii_digit()) {
            return Ok(identifier.into());
        }
        self.lookup_user_by_username(identifier)?
            .map(|user| user.id)
            .ok_or(SourceError::NotFound)
    }

    fn lookup_user_from_statuses(&self, user_id: &str) -> Result<Option<User>, SourceError> {
        let mut url = self.api_url(&["accounts", user_id, "statuses"]);
        url.query_pairs_mut().append_pair("limit", "1");
        let statuses: Vec<ApiStatusAccount> = self.transport.get_json(url)?;
        Ok(statuses
            .into_iter()
            .next()
            .map(|status| status.account.into()))
    }
}
