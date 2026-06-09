pub mod activitypub;
pub mod atproto;
pub(crate) mod common;
pub mod feed;
pub(crate) mod hacker_news;
pub mod lemmy;
pub mod mastodon;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParseType {
    ActivityPub,
    AtProto,
    Feed,
    Lemmy,
    Mastodon,
}
