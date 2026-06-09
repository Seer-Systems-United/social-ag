mod support;

use social_ag::{ActivityPub, SocialSource};
use support::{Response, server};

#[test]
fn falls_back_to_a_profile_json_feed() {
    let base = server(4, |target, base| match target {
        "/users/alice" => Response::new(
            "application/activity+json",
            format!(
                r#"{{"id":"{base}users/alice","preferredUsername":"alice",
                "url":"{base}@alice","outbox":"{base}users/alice/outbox"}}"#
            ),
        ),
        "/users/alice/outbox" => Response::new("application/activity+json", "{}"),
        "/@alice" => Response::new(
            "text/html",
            format!(
                r#"<link rel="alternate" type="application/feed+json" href="{base}alice.json">"#
            ),
        ),
        "/alice.json" => Response::new(
            "application/feed+json",
            format!(
                r#"{{"version":"https://jsonfeed.org/version/1","items":[{{
                "id":"1","url":"{base}posts/1","date_published":"2026-06-08T20:00:00Z",
                "content_html":"<p>Feed fallback</p>"}}]}}"#
            ),
        ),
        _ => panic!("unexpected target {target}"),
    });
    let source = ActivityPub::new(&base).unwrap();
    let post = source
        .try_fetch_latest_post_by_user(&format!("{base}users/alice"))
        .unwrap()
        .unwrap();

    assert_eq!(post.content.as_deref(), Some("Feed fallback"));
}
