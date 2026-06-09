pub(crate) const CAPABILITIES: &[crate::Capability] = &[
    crate::Capability::LookupUserById,
    crate::Capability::LookupUserByUsername,
    crate::Capability::LookupUserByDisplayName,
    crate::Capability::FetchUserPosts,
];

macro_rules! configurable_source {
    ($name:ident, $default_instance_url:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            parser: $crate::parse::lemmy::Parser,
        }

        impl $name {
            pub fn new(instance_url: impl AsRef<str>) -> Result<Self, $crate::SourceError> {
                Self::new_with_config(instance_url, $crate::TransportConfig::default())
            }

            pub fn new_with_config(
                instance_url: impl AsRef<str>,
                config: $crate::TransportConfig,
            ) -> Result<Self, $crate::SourceError> {
                Ok(Self {
                    parser: $crate::parse::lemmy::Parser::new(instance_url, config)?,
                })
            }

            pub fn with_access_token(mut self, token: impl Into<String>) -> Self {
                self.parser = self.parser.with_access_token(token);
                self
            }

            pub fn instance_url(&self) -> &reqwest::Url {
                self.parser.instance_url()
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new($default_instance_url).unwrap()
            }
        }

        $crate::sources::social_source_impl!(
            $name,
            instance_url,
            $crate::ParseType::Lemmy,
            $crate::Authentication::OptionalBearer,
            $crate::sources::lemmy::CAPABILITIES,
            &[]
        );
    };
}

pub(crate) use configurable_source;
