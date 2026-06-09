macro_rules! fixed_feed_source {
    ($name:ident, $feed:expr, $id:expr, $display:expr, $profile:expr, $quirks:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            parser: $crate::parse::feed::Parser,
        }

        impl $name {
            pub fn new() -> Result<Self, $crate::SourceError> {
                Self::new_with_config($crate::TransportConfig::default())
            }

            pub fn new_with_config(
                config: $crate::TransportConfig,
            ) -> Result<Self, $crate::SourceError> {
                let user = $crate::User {
                    id: $id.into(),
                    username: $id.into(),
                    display_name: Some($display.into()),
                    profile_url: $profile.into(),
                };
                Ok(Self {
                    parser: $crate::parse::feed::Parser::new($feed, user, config)?,
                })
            }

            pub fn feed_url(&self) -> &reqwest::Url {
                self.parser.feed_url()
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new().unwrap()
            }
        }

        $crate::sources::feed::feed_social_source!(
            $name,
            $crate::Authentication::None,
            &[
                $crate::Capability::LookupUserById,
                $crate::Capability::LookupUserByUsername,
                $crate::Capability::LookupUserByDisplayName,
                $crate::Capability::FetchUserPosts,
            ],
            $quirks
        );
    };
}

pub(crate) use fixed_feed_source;
