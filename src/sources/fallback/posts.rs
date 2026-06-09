use crate::{Post, SocialSource, SourceError};

use super::{FallbackChain, policy::allows_fallback};

impl FallbackChain {
    pub fn try_fetch_latest_post_by_user(&self, id: &str) -> Result<Option<Post>, SourceError> {
        self.first_optional(|source| source.try_fetch_latest_post_by_user(id))
    }

    pub fn try_fetch_last_posts_by_user(
        &self,
        id: &str,
        count: usize,
    ) -> Result<Vec<Post>, SourceError> {
        self.first_result(|source| source.try_fetch_last_posts_by_user(id, count))
    }

    fn first_result<T>(
        &self,
        mut operation: impl FnMut(&dyn SocialSource) -> Result<T, SourceError>,
    ) -> Result<T, SourceError> {
        let mut last_error = None;
        for source in &self.sources {
            match operation(source.as_ref()) {
                Ok(value) => return Ok(value),
                Err(error) if allows_fallback(&error) => last_error = Some(error),
                Err(error) => return Err(error),
            }
        }
        Err(last_error.unwrap_or(SourceError::Unsupported {
            capability: "an empty fallback chain",
        }))
    }
}
