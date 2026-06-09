use crate::{SourceError, parse::common::required_identifier, sources::User};

use super::{
    Parser,
    models::{PersonDetails, SearchResponse},
};

impl Parser {
    pub(crate) fn lookup_user_by_id(&self, user_id: &str) -> Result<Option<User>, SourceError> {
        let user_id = required_identifier(user_id)?;
        if !user_id.chars().all(|value| value.is_ascii_digit()) {
            return Err(SourceError::InvalidIdentifier(user_id.into()));
        }
        self.lookup_person("person_id", user_id)
    }

    pub(crate) fn lookup_user_by_username(
        &self,
        username: &str,
    ) -> Result<Option<User>, SourceError> {
        self.lookup_person(
            "username",
            required_identifier(username)?.trim_start_matches('@'),
        )
    }

    pub(crate) fn lookup_user_by_display_name(
        &self,
        display_name: &str,
    ) -> Result<Option<User>, SourceError> {
        let display_name = required_identifier(display_name)?;
        let mut url = self.api_url("search");
        url.query_pairs_mut()
            .append_pair("q", display_name)
            .append_pair("type_", "Users")
            .append_pair("sort", "New")
            .append_pair("limit", "50");
        let response: SearchResponse = self.transport.get_json(url)?;
        Ok(response
            .users
            .into_iter()
            .map(|view| view.person)
            .find(|person| {
                person
                    .display_name
                    .as_deref()
                    .is_some_and(|name| name.eq_ignore_ascii_case(display_name))
            })
            .map(User::from))
    }

    pub(super) fn resolve_person(&self, value: &str) -> Result<User, SourceError> {
        let user = if value.chars().all(|character| character.is_ascii_digit()) {
            self.lookup_user_by_id(value)?
        } else {
            self.lookup_user_by_username(value)?
        };
        user.ok_or(SourceError::NotFound)
    }

    fn lookup_person(&self, parameter: &str, value: &str) -> Result<Option<User>, SourceError> {
        let mut url = self.api_url("user");
        url.query_pairs_mut().append_pair(parameter, value);
        match self.transport.get_json::<PersonDetails>(url) {
            Ok(response) => Ok(Some(response.person_view.person.into())),
            Err(SourceError::NotFound) => Ok(None),
            Err(error) => Err(error),
        }
    }
}
