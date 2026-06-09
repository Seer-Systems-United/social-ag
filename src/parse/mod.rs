pub mod activitypub;
pub mod atproto;
pub(crate) mod common;
pub mod feed;
pub(crate) mod json_ld;
pub mod lemmy;
pub mod mastodon;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParseType {
    ActivityPub,
    AtProto,
    Facebook,
    Feed,
    Lemmy,
    Mastodon,
    PublicHtml,
    PublicJson,
    Twitter,
}
