macro_rules! feed_source {
    ($name:ident, $feed_url_template:expr, $profile_url_template:expr) => {
        feed_source!($name, $feed_url_template, $profile_url_template, $crate::Authentication::None, &[
            $crate::Capability::LookupUserById,
            $crate::Capability::LookupUserByUsername,
            $crate::Capability::LookupUserByDisplayName,
            $crate::Capability::FetchUserPosts,
        ]);
    };
    ($name:ident, $feed_url_template:expr, $profile_url_template:expr, $auth:expr, $caps:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            parser: $crate::parse::feed::Parser,
        }

        impl $name {
            pub fn new(username: impl AsRef<str>) -> Result<Self, $crate::SourceError> {
                Self::new_with_config(username, $crate::TransportConfig::default())
            }

            pub fn new_with_config(
                username: impl AsRef<str>,
                config: $crate::TransportConfig,
            ) -> Result<Self, $crate::SourceError> {
                let username = username.as_ref().trim_start_matches('@').to_string();
                let feed_url = format!($feed_url_template, username = username);
                let profile_url = format!($profile_url_template, username = username);
                let user = $crate::User {
                    id: username.clone(),
                    username,
                    display_name: None,
                    profile_url,
                };
                Ok(Self { parser: $crate::parse::feed::Parser::new(feed_url, user, config)? })
            }

            pub fn feed_url(&self) -> &reqwest::Url {
                self.parser.feed_url()
            }
        }

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

            fn try_lookup_user_by_id(&self, id: &str) -> Result<Option<$crate::User>, $crate::SourceError> {
                Ok(self.parser.lookup_user_by_id(id))
            }

            fn try_lookup_user_by_username(&self, username: &str) -> Result<Option<$crate::User>, $crate::SourceError> {
                Ok(self.parser.lookup_user_by_username(username))
            }

            fn try_lookup_user_by_display_name(&self, name: &str) -> Result<Option<$crate::User>, $crate::SourceError> {
                Ok(self.parser.lookup_user_by_display_name(name))
            }

            fn try_fetch_latest_post_by_user(&self, id: &str) -> Result<Option<$crate::Post>, $crate::SourceError> {
                if self.parser.lookup_user_by_id(id).is_none() && self.parser.lookup_user_by_username(id).is_none() {
                    return Ok(None);
                }
                self.parser.fetch_last_posts(1).map(|posts| posts.into_iter().next())
            }

            fn try_fetch_last_posts_by_user(&self, id: &str, count: usize) -> Result<Vec<$crate::Post>, $crate::SourceError> {
                if self.parser.lookup_user_by_id(id).is_none() && self.parser.lookup_user_by_username(id).is_none() {
                    return Ok(Vec::new());
                }
                self.parser.fetch_last_posts(count)
            }
        }
    };
}

pub(crate) use feed_source;
