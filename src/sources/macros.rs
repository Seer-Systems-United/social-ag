macro_rules! social_source_impl {
    ($name:ident, $url_method:ident, $protocol:expr, $auth:expr, $caps:expr, $quirks:expr) => {
        impl $crate::SocialSource for $name {
            fn definition(&self) -> $crate::SourceDefinition {
                $crate::SourceDefinition {
                    name: stringify!($name),
                    base_url: self.parser.$url_method().clone(),
                    protocol: $protocol,
                    authentication: $auth,
                    capabilities: $caps,
                    quirks: $quirks,
                }
            }

            fn try_lookup_user_by_id(
                &self,
                id: &str,
            ) -> Result<Option<$crate::User>, $crate::SourceError> {
                self.parser.lookup_user_by_id(id)
            }

            fn try_lookup_user_by_username(
                &self,
                username: &str,
            ) -> Result<Option<$crate::User>, $crate::SourceError> {
                self.parser.lookup_user_by_username(username)
            }

            fn try_lookup_user_by_display_name(
                &self,
                name: &str,
            ) -> Result<Option<$crate::User>, $crate::SourceError> {
                self.parser.lookup_user_by_display_name(name)
            }

            fn try_fetch_latest_post_by_user(
                &self,
                id: &str,
            ) -> Result<Option<$crate::Post>, $crate::SourceError> {
                self.parser.fetch_latest_post_by_user(id)
            }

            fn try_fetch_last_posts_by_user(
                &self,
                id: &str,
                count: usize,
            ) -> Result<Vec<$crate::Post>, $crate::SourceError> {
                self.parser.fetch_last_posts_by_user(id, count)
            }
        }
    };
}

pub(crate) use social_source_impl;
