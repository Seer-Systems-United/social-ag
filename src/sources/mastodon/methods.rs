macro_rules! source_methods {
    () => {
        pub fn with_access_token(mut self, token: impl Into<String>) -> Self {
            self.parser = self.parser.with_access_token(token);
            self
        }

        pub fn instance_url(&self) -> &reqwest::Url {
            self.parser.instance_url()
        }

        pub fn probe_instance(
            &self,
        ) -> Result<$crate::parse::mastodon::InstanceMetadata, $crate::SourceError> {
            self.parser.probe_instance()
        }

        pub fn try_lookup_user_by_id(
            &self,
            id: &str,
        ) -> Result<Option<$crate::User>, $crate::SourceError> {
            self.parser.lookup_user_by_id(id)
        }

        pub fn try_lookup_user_by_username(
            &self,
            username: &str,
        ) -> Result<Option<$crate::User>, $crate::SourceError> {
            self.parser.lookup_user_by_username(username)
        }

        pub fn try_lookup_user_by_display_name(
            &self,
            name: &str,
        ) -> Result<Option<$crate::User>, $crate::SourceError> {
            self.parser.lookup_user_by_display_name(name)
        }

        pub fn try_fetch_latest_post_by_user(
            &self,
            id: &str,
        ) -> Result<Option<$crate::Post>, $crate::SourceError> {
            self.parser.fetch_latest_post_by_user(id)
        }

        pub fn try_fetch_last_posts_by_user(
            &self,
            id: &str,
            count: usize,
        ) -> Result<Vec<$crate::Post>, $crate::SourceError> {
            self.parser.fetch_last_posts_by_user(id, count)
        }
    };
}

pub(crate) use source_methods;
