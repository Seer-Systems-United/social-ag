macro_rules! feed_constructor {
    ($name:ident, $feed_url_template:expr, $profile_url_template:expr) => {
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
                Ok(Self {
                    parser: $crate::parse::feed::Parser::new(feed_url, user, config)?,
                })
            }

            pub fn feed_url(&self) -> &reqwest::Url {
                self.parser.feed_url()
            }
        }
    };
}

pub(crate) use feed_constructor;
