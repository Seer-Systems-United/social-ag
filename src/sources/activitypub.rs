pub(crate) mod config;

pub(crate) const CAPABILITIES: &[crate::Capability] = &[
    crate::Capability::LookupUserById,
    crate::Capability::LookupUserByUsername,
    crate::Capability::FetchUserPosts,
];

macro_rules! configurable_source {
    ($name:ident) => {
        $crate::sources::activitypub::configurable_source_impl!($name);
    };
    ($name:ident, $default_instance_url:expr) => {
        $crate::sources::activitypub::configurable_source_impl!($name);

        impl Default for $name {
            fn default() -> Self {
                Self::new($default_instance_url).unwrap()
            }
        }
    };
}

macro_rules! configurable_source_impl {
    ($name:ident) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            parser: $crate::parse::activitypub::Parser,
        }

        impl $name {
            pub fn new(instance_url: impl AsRef<str>) -> Result<Self, $crate::SourceError> {
                Self::new_with_config(
                    instance_url,
                    $crate::sources::activitypub::config::transport_config(),
                )
            }

            pub fn new_with_config(
                instance_url: impl AsRef<str>,
                config: $crate::TransportConfig,
            ) -> Result<Self, $crate::SourceError> {
                Ok(Self {
                    parser: $crate::parse::activitypub::Parser::new(instance_url, config)?,
                })
            }

            pub fn with_access_token(mut self, token: impl Into<String>) -> Self {
                self.parser = self.parser.with_access_token(token);
                self
            }

            pub fn instance_url(&self) -> &reqwest::Url {
                self.parser.instance_url()
            }

            pub fn try_lookup_user_by_id(
                &self,
                user_id: &str,
            ) -> Result<Option<$crate::User>, $crate::SourceError> {
                self.parser.lookup_user_by_id(user_id)
            }

            pub fn try_lookup_user_by_username(
                &self,
                username: &str,
            ) -> Result<Option<$crate::User>, $crate::SourceError> {
                self.parser.lookup_user_by_username(username)
            }

            pub fn try_fetch_last_posts_by_user(
                &self,
                user_id: &str,
                count: usize,
            ) -> Result<Vec<$crate::Post>, $crate::SourceError> {
                self.parser.fetch_last_posts_by_user(user_id, count)
            }
        }

        $crate::sources::social_source_impl!(
            $name,
            instance_url,
            $crate::ParseType::ActivityPub,
            $crate::Authentication::OptionalBearer,
            $crate::sources::activitypub::CAPABILITIES,
            &[]
        );
    };
}

pub(crate) use configurable_source;
pub(crate) use configurable_source_impl;
