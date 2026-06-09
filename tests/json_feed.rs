mod support;

use social_ag::{FeedSource, SocialSource, User};
use support::{Response, server};

#[test]
fn parses_json_feed() {
    let base = server(1, |_, base| {
        Response::new(
            "application/feed+json",
            format!(
                r#"{{"version":"https://jsonfeed.org/version/1","items":[{{
                "id":"entry-1","url":"{base}entry","title":"Update",
                "date_published":"2026-06-08T20:15:30Z",
                "content_text":"JSON Feed body"}}]}}"#
            ),
        )
    });
    let publisher = User {
        id: "example".into(),
        username: "example".into(),
        display_name: None,
        profile_url: base.clone(),
    };
    let source = FeedSource::new(format!("{base}feed.json"), publisher).unwrap();
    let post = source.fetch_latest_post_by_user("example").unwrap();

    assert_eq!(post.title.as_deref(), Some("Update"));
    assert_eq!(post.content.as_deref(), Some("JSON Feed body"));
}
