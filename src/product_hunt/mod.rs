use crate::{SourceQuirk, sources::feed::fixed_feed_source};

fixed_feed_source!(
    ProductHunt,
    "https://www.producthunt.com/feed",
    "producthunt",
    "Product Hunt",
    "https://www.producthunt.com/",
    &[SourceQuirk::CommunityAsUser]
);
