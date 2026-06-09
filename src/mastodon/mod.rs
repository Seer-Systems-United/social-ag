pub use crate::parse::mastodon::{InstanceMetadata, MastodonError};

use crate::sources::mastodon;

mastodon::configurable_source!(
    Mastodon,
    "https://mastodon.social/",
    &[crate::SourceQuirk::MastodonApiCompatible]
);
