mod support;

use social_ag::{Lemmy, SocialSource};
use support::{Response, server};

#[test]
fn parses_users_posts_and_communities() {
    let base = server(3, |target, base| {
        let posts = if target.contains("person_id=5") {
            format!(
                r#"[{{
                    "post":{{"id":10,"name":"Release update","body":"Lemmy notes",
                        "url":null,"published":"2026-06-03T08:17:23Z","ap_id":"{base}post/10"}},
                    "creator":{{"id":5,"name":"alice","display_name":"Alice",
                        "actor_id":"{base}u/alice"}},
                    "community":{{"id":2,"name":"announcements","title":"Announcements",
                        "actor_id":"{base}c/announcements"}}
                }}]"#
            )
        } else {
            "[]".into()
        };
        Response::new(
            "application/json",
            format!(
                r#"{{
                "person_view":{{"person":{{"id":5,"name":"alice","display_name":"Alice",
                    "actor_id":"{base}u/alice"}}}},
                "posts":{posts}
            }}"#
            ),
        )
    });
    let source = Lemmy::new(&base).unwrap();
    let user = source.lookup_user_by_username("alice").unwrap();
    let post = source.fetch_latest_post_by_user("alice").unwrap();

    assert_eq!(user.id, "5");
    assert_eq!(post.title.as_deref(), Some("Release update"));
    assert_eq!(post.community.unwrap().name, "announcements");
}
