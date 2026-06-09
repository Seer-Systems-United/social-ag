use crate::{SocialSource, SourceError, User};

use super::{FallbackChain, policy::allows_fallback};

impl FallbackChain {
    pub fn try_lookup_user_by_id(&self, id: &str) -> Result<Option<User>, SourceError> {
        self.first_optional(|source| source.try_lookup_user_by_id(id))
    }

    pub fn try_lookup_user_by_username(&self, username: &str) -> Result<Option<User>, SourceError> {
        self.first_optional(|source| source.try_lookup_user_by_username(username))
    }

    pub fn try_lookup_user_by_display_name(&self, name: &str) -> Result<Option<User>, SourceError> {
        self.first_optional(|source| source.try_lookup_user_by_display_name(name))
    }

    pub(super) fn first_optional<T>(
        &self,
        mut operation: impl FnMut(&dyn SocialSource) -> Result<Option<T>, SourceError>,
    ) -> Result<Option<T>, SourceError> {
        let mut last_error = None;
        for source in &self.sources {
            match operation(source.as_ref()) {
                Ok(Some(value)) => return Ok(Some(value)),
                Ok(None) => last_error = Some(SourceError::NotFound),
                Err(error) if allows_fallback(&error) => last_error = Some(error),
                Err(error) => return Err(error),
            }
        }
        match last_error {
            Some(SourceError::NotFound) => Ok(None),
            Some(error) => Err(error),
            None => Err(SourceError::Unsupported {
                capability: "an empty fallback chain",
            }),
        }
    }
}
