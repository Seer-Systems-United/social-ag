mod support;

use social_ag::Mastodon;
use support::{Response, server};

#[test]
fn parses_accounts_and_statuses() {
    let base = server(3, |target, base| {
        if target.starts_with("/api/v1/accounts/lookup") {
            return Response::new(
                "application/json",
                format!(
                    r#"{{"id":"42","username":"alice","display_name":"Alice","url":"{base}@alice"}}"#
                ),
            );
        }
        assert!(target.starts_with("/api/v1/accounts/42/statuses"));
        Response::new(
            "application/json",
            format!(
                r#"[{{
                "id":"123",
                "created_at":"2026-06-08T20:15:30Z",
                "spoiler_text":"Release notes",
                "content":"<p>Hello <strong>Fediverse</strong>.</p><p>Second paragraph.</p>",
                "url":"{base}@alice/123",
                "uri":"{base}users/alice/statuses/123",
                "in_reply_to_id":null,
                "account":{{
                    "id":"42",
                    "username":"alice",
                    "display_name":"Alice",
                    "url":"{base}@alice"
                }}
            }}]"#
            ),
        )
    });
    let source = Mastodon::new(&base).unwrap();
    let user = source
        .try_lookup_user_by_username("alice")
        .unwrap()
        .unwrap();
    let post = source
        .try_fetch_latest_post_by_user("alice")
        .unwrap()
        .unwrap();

    assert_eq!(user.id, "42");
    assert_eq!(post.publisher_user, user);
    assert_eq!(
        post.content.as_deref(),
        Some("Hello Fediverse.\n\nSecond paragraph.")
    );
}

#[test]
fn rejects_non_http_instances() {
    assert!(Mastodon::new("file:///tmp/mastodon").is_err());
}
