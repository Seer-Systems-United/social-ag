macro_rules! feed_social_source {
    ($name:ident, $auth:expr, $caps:expr) => {
        impl $crate::SocialSource for $name {
            fn definition(&self) -> $crate::SourceDefinition {
                $crate::SourceDefinition {
                    name: stringify!($name),
                    base_url: self.parser.feed_url().clone(),
                    protocol: $crate::ParseType::Feed,
                    authentication: $auth,
                    capabilities: $caps,
                    quirks: &[],
                }
            }

            fn try_lookup_user_by_id(
                &self,
                id: &str,
            ) -> Result<Option<$crate::User>, $crate::SourceError> {
                Ok(self.parser.lookup_user_by_id(id))
            }

            fn try_lookup_user_by_username(
                &self,
                username: &str,
            ) -> Result<Option<$crate::User>, $crate::SourceError> {
                Ok(self.parser.lookup_user_by_username(username))
            }

            fn try_lookup_user_by_display_name(
                &self,
                name: &str,
            ) -> Result<Option<$crate::User>, $crate::SourceError> {
                Ok(self.parser.lookup_user_by_display_name(name))
            }

            fn try_fetch_latest_post_by_user(
                &self,
                id: &str,
            ) -> Result<Option<$crate::Post>, $crate::SourceError> {
                Ok(self.try_fetch_last_posts_by_user(id, 1)?.into_iter().next())
            }

            fn try_fetch_last_posts_by_user(
                &self,
                id: &str,
                count: usize,
            ) -> Result<Vec<$crate::Post>, $crate::SourceError> {
                if self.parser.lookup_user_by_id(id).is_none()
                    && self.parser.lookup_user_by_username(id).is_none()
                {
                    return Ok(Vec::new());
                }
                self.parser.fetch_last_posts(count)
            }
        }
    };
}

pub(crate) use feed_social_source;
