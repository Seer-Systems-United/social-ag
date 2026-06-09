use atrium_api::{
    app::bsky::{
        actor::defs::{ProfileView, ProfileViewBasic, ProfileViewDetailed},
        feed::{defs::PostView, post},
    },
    types::TryFromUnknown,
};

use crate::{
    SourceError,
    parse::common::optional_text,
    sources::{Post, User},
};

use super::Parser;

impl Parser {
    pub(super) fn post_from_view(&self, post: PostView) -> Result<Post, SourceError> {
        let atrium_api::types::Object { data: post, .. } = post;
        let record_key = post
            .uri
            .rsplit('/')
            .next()
            .ok_or_else(|| SourceError::InvalidResponse("AT URI has no record key".into()))?;
        let user = self.user_from_basic_profile(post.author);
        let url = self
            .web_url
            .join(&format!("profile/{}/post/{record_key}", user.username))
            .unwrap()
            .to_string();
        let record = post::Record::try_from_unknown(post.record)
            .map_err(|error| SourceError::InvalidResponse(error.to_string()))?;
        let atrium_api::types::Object { data: record, .. } = record;
        Ok(Post {
            id: post.uri,
            publisher_user: user,
            title: None,
            content: optional_text(record.text),
            timestamp: record.created_at.as_ref().with_timezone(&chrono::Utc),
            url,
            community: None,
            in_reply_to_id: record.reply.map(|reply| reply.data.parent.data.uri),
        })
    }

    pub(super) fn user_from_detailed_profile(&self, value: ProfileViewDetailed) -> User {
        let atrium_api::types::Object { data, .. } = value;
        self.user_from_parts(data.did.as_str(), data.handle.as_str(), data.display_name)
    }

    pub(super) fn user_from_profile_view(&self, value: ProfileView) -> User {
        let atrium_api::types::Object { data, .. } = value;
        self.user_from_parts(data.did.as_str(), data.handle.as_str(), data.display_name)
    }

    fn user_from_basic_profile(&self, value: ProfileViewBasic) -> User {
        let atrium_api::types::Object { data, .. } = value;
        self.user_from_parts(data.did.as_str(), data.handle.as_str(), data.display_name)
    }

    fn user_from_parts(&self, did: &str, handle: &str, display_name: Option<String>) -> User {
        User {
            id: did.into(),
            username: handle.into(),
            display_name: display_name.and_then(optional_text),
            profile_url: self
                .web_url
                .join(&format!("profile/{handle}"))
                .unwrap()
                .to_string(),
        }
    }
}
