use serde::Deserialize;

use crate::{
    SourceError,
    parse::common::{optional_text, parse_datetime},
    sources::{Community, Post, User},
};

#[derive(Debug, Deserialize)]
pub(super) struct PersonDetails {
    pub(super) person_view: PersonView,
    #[serde(default)]
    pub(super) posts: Vec<PostView>,
}

#[derive(Debug, Deserialize)]
pub(super) struct SearchResponse {
    #[serde(default)]
    pub(super) users: Vec<PersonView>,
}

#[derive(Debug, Deserialize)]
pub(super) struct PersonView {
    pub(super) person: Person,
}

#[derive(Debug, Deserialize)]
pub(super) struct Person {
    id: u64,
    name: String,
    pub(super) display_name: Option<String>,
    actor_id: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct PostView {
    post: LemmyPost,
    creator: Person,
    community: LemmyCommunity,
}

#[derive(Debug, Deserialize)]
struct LemmyPost {
    id: u64,
    name: String,
    body: Option<String>,
    url: Option<String>,
    published: String,
    ap_id: String,
}

#[derive(Debug, Deserialize)]
struct LemmyCommunity {
    id: u64,
    name: String,
    title: String,
    actor_id: String,
}

impl From<Person> for User {
    fn from(person: Person) -> Self {
        Self {
            id: person.id.to_string(),
            username: person.name,
            display_name: person.display_name.and_then(optional_text),
            profile_url: person.actor_id,
        }
    }
}

impl TryFrom<PostView> for Post {
    type Error = SourceError;

    fn try_from(view: PostView) -> Result<Self, Self::Error> {
        let content = view
            .post
            .body
            .and_then(optional_text)
            .or_else(|| view.post.url.and_then(optional_text));
        Ok(Self {
            id: view.post.id.to_string(),
            publisher_user: view.creator.into(),
            title: optional_text(view.post.name),
            content,
            timestamp: parse_datetime(&view.post.published)?,
            url: view.post.ap_id,
            community: Some(Community {
                id: view.community.id.to_string(),
                name: view.community.name,
                display_name: optional_text(view.community.title),
                url: view.community.actor_id,
            }),
            in_reply_to_id: None,
        })
    }
}
