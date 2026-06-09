use atrium_api::app::bsky::actor::{get_profile, search_actors};

use crate::{SourceError, parse::common::required_identifier, sources::User};

use super::{Parser, identifier::parse_actor_identifier};

impl Parser {
    pub(crate) fn lookup_user_by_id(&self, user_id: &str) -> Result<Option<User>, SourceError> {
        self.lookup_profile(user_id)
    }

    pub(crate) fn lookup_user_by_username(
        &self,
        username: &str,
    ) -> Result<Option<User>, SourceError> {
        self.lookup_profile(username.trim_start_matches('@'))
    }

    pub(crate) fn lookup_user_by_display_name(
        &self,
        display_name: &str,
    ) -> Result<Option<User>, SourceError> {
        let display_name = required_identifier(display_name)?;
        let mut url = self.xrpc_url(search_actors::NSID);
        url.query_pairs_mut()
            .append_pair("q", display_name)
            .append_pair("limit", "25");
        let response: search_actors::Output = self.transport.get_json(url)?;
        let atrium_api::types::Object { data: response, .. } = response;
        Ok(response
            .actors
            .into_iter()
            .find(|actor| {
                actor
                    .display_name
                    .as_deref()
                    .is_some_and(|name| name.eq_ignore_ascii_case(display_name))
            })
            .map(|profile| self.user_from_profile_view(profile)))
    }

    pub(super) fn lookup_profile(&self, actor: &str) -> Result<Option<User>, SourceError> {
        let actor = parse_actor_identifier(actor)?;
        let mut url = self.xrpc_url(get_profile::NSID);
        url.query_pairs_mut().append_pair("actor", actor.as_ref());
        match self.transport.get_json::<get_profile::Output>(url) {
            Ok(profile) => Ok(Some(self.user_from_detailed_profile(profile))),
            Err(SourceError::NotFound)
            | Err(SourceError::Http {
                status: reqwest::StatusCode::BAD_REQUEST,
            }) => Ok(None),
            Err(error) => Err(error),
        }
    }
}
