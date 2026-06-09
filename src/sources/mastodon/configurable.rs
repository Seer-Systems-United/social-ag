macro_rules! configurable_source {
    ($name:ident, $default_url:expr, $quirks:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            parser: $crate::parse::mastodon::Parser,
        }

        impl $name {
            pub fn new(url: impl AsRef<str>) -> Result<Self, $crate::SourceError> {
                Self::new_with_config(url, $crate::TransportConfig::default())
            }

            pub fn new_with_config(
                url: impl AsRef<str>,
                config: $crate::TransportConfig,
            ) -> Result<Self, $crate::SourceError> {
                Ok(Self {
                    parser: $crate::parse::mastodon::Parser::new(url, config, $quirks)?,
                })
            }

            $crate::sources::mastodon::source_methods!();
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new($default_url).unwrap()
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

pub(crate) use configurable_source;
