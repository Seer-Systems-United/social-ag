pub(crate) mod activitypub;
pub(crate) mod atproto;
mod definition;
pub mod fallback;
pub(crate) mod feed;
pub(crate) mod lemmy;
mod macros;
pub(crate) mod mastodon;
mod models;
pub mod registry;
mod source;

pub use definition::{Authentication, Capability, SourceDefinition, SourceQuirk};
pub(crate) use macros::social_source_impl;
pub use models::{Community, Post, User};
pub use source::SocialSource;
