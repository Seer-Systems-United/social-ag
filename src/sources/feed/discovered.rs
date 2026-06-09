macro_rules! profile_feed_source {
    ($name:ident, $profile_url_template:expr) => {
        $crate::sources::feed::profile_feed_source!(
            $name,
            $profile_url_template,
            $crate::parse::feed::alternate_feed
        );
    };
    ($name:ident, $profile_url_template:expr, $resolver:path) => {
        #[derive(Clone)]
        pub struct $name {
            parser: $crate::parse::feed::DiscoveryParser,
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
                let profile_url = format!($profile_url_template, username = username);
                let user = $crate::User {
                    id: username.clone(),
                    username,
                    display_name: None,
                    profile_url: profile_url.clone(),
                };
                Ok(Self {
                    parser: $crate::parse::feed::DiscoveryParser::new(
                        profile_url,
                        user,
                        config,
                        $resolver,
                    )?,
                })
            }

            pub fn profile_url(&self) -> &reqwest::Url {
                self.parser.profile_url()
            }
        }

        $crate::sources::social_source_impl!(
            $name,
            profile_url,
            $crate::ParseType::Feed,
            $crate::Authentication::None,
            &[
                $crate::Capability::LookupUserById,
                $crate::Capability::LookupUserByUsername,
                $crate::Capability::FetchUserPosts,
            ],
            &[]
        );
    };
}

pub(crate) use profile_feed_source;
