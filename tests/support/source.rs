use reqwest::{StatusCode, Url};
use social_ag::{
    Authentication, Capability, ParseType, Post, SocialSource, SourceDefinition, SourceError, User,
};

pub struct TestSource {
    pub blocked: bool,
}

impl SocialSource for TestSource {
    fn definition(&self) -> SourceDefinition {
        SourceDefinition {
            name: "TestSource",
            base_url: Url::parse("https://example.social/").unwrap(),
            protocol: ParseType::ActivityPub,
            authentication: Authentication::None,
            capabilities: &[Capability::LookupUserByUsername],
            quirks: &[],
        }
    }

    fn try_lookup_user_by_id(&self, _: &str) -> Result<Option<User>, SourceError> {
        Ok(None)
    }

    fn try_lookup_user_by_username(&self, username: &str) -> Result<Option<User>, SourceError> {
        if self.blocked {
            return Err(SourceError::Blocked {
                status: StatusCode::FORBIDDEN,
            });
        }
        Ok(Some(User {
            id: "1".into(),
            username: username.into(),
            display_name: None,
            profile_url: "https://example.social/@alice".into(),
        }))
    }

    fn try_lookup_user_by_display_name(&self, _: &str) -> Result<Option<User>, SourceError> {
        Ok(None)
    }

    fn try_fetch_latest_post_by_user(&self, _: &str) -> Result<Option<Post>, SourceError> {
        Ok(None)
    }

    fn try_fetch_last_posts_by_user(&self, _: &str, _: usize) -> Result<Vec<Post>, SourceError> {
        Ok(Vec::new())
    }
}
