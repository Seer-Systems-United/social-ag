macro_rules! source_methods {
    () => {
        pub fn with_access_token(mut self, access_token: impl Into<String>) -> Self {
            self.parser = self.parser.with_access_token(access_token);
            self
        }

        pub fn instance_url(&self) -> &reqwest::Url {
            self.parser.instance_url()
        }

        pub fn try_lookup_user_by_id(
            &self,
            user_id: &str,
        ) -> Result<Option<$crate::sources::User>, $crate::mastodon::MastodonError> {
            self.parser.lookup_user_by_id(user_id)
        }

        pub fn try_lookup_user_by_username(
            &self,
            username: &str,
        ) -> Result<Option<$crate::sources::User>, $crate::mastodon::MastodonError> {
            self.parser.lookup_user_by_username(username)
        }

        pub fn try_lookup_user_by_display_name(
            &self,
            display_name: &str,
        ) -> Result<Option<$crate::sources::User>, $crate::mastodon::MastodonError> {
            self.parser.lookup_user_by_display_name(display_name)
        }

        pub fn try_fetch_latest_post_by_user(
            &self,
            user_id: &str,
        ) -> Result<Option<$crate::sources::Post>, $crate::mastodon::MastodonError> {
            self.parser.fetch_latest_post_by_user(user_id)
        }

        pub fn try_fetch_last_posts_by_user(
            &self,
            user_id: &str,
            count: usize,
        ) -> Result<Vec<$crate::sources::Post>, $crate::mastodon::MastodonError> {
            self.parser.fetch_last_posts_by_user(user_id, count)
        }
    };
}

macro_rules! social_source_impl {
    ($name:ident) => {
        impl $crate::sources::SocialSource for $name {
            fn parse_type(&self) -> $crate::parse::ParseType {
                $crate::parse::ParseType::Mastodon
            }

            fn lookup_user_by_id(&self, user_id: &str) -> Option<$crate::sources::User> {
                self.try_lookup_user_by_id(user_id).ok().flatten()
            }

            fn lookup_user_by_username(&self, username: &str) -> Option<$crate::sources::User> {
                self.try_lookup_user_by_username(username).ok().flatten()
            }

            fn lookup_user_by_display_name(
                &self,
                display_name: &str,
            ) -> Option<$crate::sources::User> {
                self.try_lookup_user_by_display_name(display_name)
                    .ok()
                    .flatten()
            }

            fn fetch_latest_post_by_user(&self, user_id: &str) -> Option<$crate::sources::Post> {
                self.try_fetch_latest_post_by_user(user_id).ok().flatten()
            }

            fn fetch_last_posts_by_user(
                &self,
                user_id: &str,
                count: usize,
            ) -> Vec<$crate::sources::Post> {
                self.try_fetch_last_posts_by_user(user_id, count)
                    .unwrap_or_default()
            }
        }
    };
}

macro_rules! configurable_source {
    ($name:ident, $default_instance_url:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            parser: $crate::parse::mastodon::Parser,
        }

        impl $name {
            pub fn new(
                instance_url: impl AsRef<str>,
            ) -> Result<Self, $crate::mastodon::MastodonError> {
                Ok(Self {
                    parser: $crate::parse::mastodon::Parser::new(instance_url)?,
                })
            }

            $crate::sources::mastodon::source_methods!();
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new($default_instance_url).unwrap()
            }
        }

        $crate::sources::mastodon::social_source_impl!($name);
    };
}

macro_rules! fixed_source {
    ($name:ident, $instance_url:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            parser: $crate::parse::mastodon::Parser,
        }

        impl $name {
            pub fn new() -> Self {
                Self::default()
            }

            $crate::sources::mastodon::source_methods!();
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    parser: $crate::parse::mastodon::Parser::new($instance_url).unwrap(),
                }
            }
        }

        $crate::sources::mastodon::social_source_impl!($name);
    };
}

pub(crate) use {configurable_source, fixed_source, social_source_impl, source_methods};
