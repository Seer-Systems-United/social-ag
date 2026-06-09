# social-ag

A Rust client for reading public posts and account data from
Mastodon-compatible platforms. The initial implementation supports configurable
Mastodon instances and Truth Social.

## Usage

```rust,no_run
use social_ag::{Mastodon, TruthSocial};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mastodon = Mastodon::default();
    let posts = mastodon.try_fetch_last_posts_by_user("Mastodon", 5)?;

    let truth_social = TruthSocial::new();
    let latest = truth_social.try_fetch_latest_post_by_user("realDonaldTrump")?;

    println!("Mastodon posts: {posts:#?}");
    println!("Latest Truth Social post: {latest:#?}");
    Ok(())
}
```

Use `Mastodon::new("https://example.social")` for another Mastodon instance.
Post fetches accept either an instance-local numeric account ID or a username.
Public accounts normally require no authentication. Authenticated lookup,
including display-name search, can be enabled with `with_access_token`.

The fallible `try_*` methods expose request and parsing errors. The
`SocialSource` trait provides convenience methods that return `None` or an empty
vector when a request fails.
