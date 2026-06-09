pub(crate) const CAPABILITIES: &[crate::Capability] = &[
    crate::Capability::LookupUserById,
    crate::Capability::LookupUserByUsername,
    crate::Capability::LookupUserByDisplayName,
    crate::Capability::FetchUserPosts,
];

macro_rules! source {
    ($name:ident, $service_url:expr, $web_url:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            parser: $crate::parse::atproto::Parser,
        }

        impl $name {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn new_with_config(
                config: $crate::TransportConfig,
            ) -> Result<Self, $crate::SourceError> {
                Self::new_with_urls($service_url, $web_url, config)
            }

            pub fn new_with_urls(
                service_url: impl AsRef<str>,
                web_url: impl AsRef<str>,
                config: $crate::TransportConfig,
            ) -> Result<Self, $crate::SourceError> {
                Ok(Self {
                    parser: $crate::parse::atproto::Parser::new(service_url, web_url, config)?,
                })
            }

            pub fn service_url(&self) -> &reqwest::Url {
                self.parser.service_url()
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

        impl Default for $name {
            fn default() -> Self {
                Self::new_with_config($crate::TransportConfig::default()).unwrap()
            }
        }

        $crate::sources::social_source_impl!(
            $name,
            service_url,
            $crate::ParseType::AtProto,
            $crate::Authentication::None,
            $crate::sources::atproto::CAPABILITIES,
            &[]
        );
    };
}

pub(crate) use source;
