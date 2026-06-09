macro_rules! fixed_source {
    ($name:ident, $url:expr, $quirks:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            parser: $crate::parse::mastodon::Parser,
        }

        impl $name {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn new_with_config(
                config: $crate::TransportConfig,
            ) -> Result<Self, $crate::SourceError> {
                Ok(Self {
                    parser: $crate::parse::mastodon::Parser::new($url, config, $quirks)?,
                })
            }

            $crate::sources::mastodon::source_methods!();
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new_with_config($crate::TransportConfig::default()).unwrap()
            }
        }

        $crate::sources::social_source_impl!(
            $name,
            instance_url,
            $crate::ParseType::Mastodon,
            $crate::Authentication::OptionalBearer,
            $crate::sources::mastodon::CAPABILITIES,
            $quirks
        );
    };
}

pub(crate) use fixed_source;
