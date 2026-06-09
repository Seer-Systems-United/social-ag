pub use crate::parse::mastodon::MastodonError;

use crate::sources::mastodon;

mastodon::configurable_source!(Mastodon, "https://mastodon.social/");
