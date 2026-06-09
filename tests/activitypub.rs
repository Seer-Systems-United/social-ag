mod support;

use social_ag::{ActivityPub, SocialSource};
use support::{Response, server};

#[test]
fn parses_actor_and_public_outbox_items() {
    let base = server(3, |target, base| match target {
        "/users/alice" => Response::new(
            "application/json",
            format!(
                r#"{{
                "id":"{base}users/alice",
                "preferredUsername":"alice",
                "name":"Alice",
                "url":"{base}@alice",
                "outbox":"{base}users/alice/outbox",
                "indexable":true
            }}"#
            ),
        ),
        "/users/alice/outbox" => Response::new(
            "application/json",
            format!(
                r#"{{
                "type":"OrderedCollection",
                "orderedItems":[
                    {{"type":"Create","object":{{
                        "id":"{base}private","type":"Note","published":"2026-06-08T19:00:00Z",
                        "to":["{base}followers"],"content":"<p>Private</p>"
                    }}}},
                    {{"type":"Create","object":{{
                        "id":"{base}public","type":"Note","published":"2026-06-08T20:00:00Z",
                        "to":["https://www.w3.org/ns/activitystreams#Public"],
                        "url":"{base}@alice/1","content":"<p>Hello ActivityPub</p>"
                    }}}}
                ]
            }}"#
            ),
        ),
        _ => panic!("unexpected target {target}"),
    });
    let source = ActivityPub::new(&base).unwrap();
    let actor_url = format!("{base}users/alice");
    let user = source.try_lookup_user_by_id(&actor_url).unwrap().unwrap();
    let post = source
        .try_fetch_latest_post_by_user(&actor_url)
        .unwrap()
        .unwrap();

    assert_eq!(user.username, "alice");
    assert_eq!(post.content.as_deref(), Some("Hello ActivityPub"));
}

#[test]
fn parses_extended_fediverse_object_types() {
    let base = server(2, |target, base| match target {
        "/users/alice" => Response::new(
            "application/activity+json",
            format!(
                r#"{{"id":"{base}users/alice","preferredUsername":"alice",
                "outbox":"{base}users/alice/outbox"}}"#
            ),
        ),
        "/users/alice/outbox" => Response::new(
            "application/activity+json",
            format!(
                r#"{{"orderedItems":[
                {{"type":"Create","object":{{"id":"{base}review","type":"Review",
                "published":"2026-06-08T20:00:00Z",
                "to":["https://www.w3.org/ns/activitystreams#Public"],
                "name":"A review","content":"<p>BookWyrm post</p>"}}}},
                {{"type":"Create","object":{{"id":"{base}audio","type":"Audio",
                "published":"2026-06-08T19:00:00Z",
                "to":["https://www.w3.org/ns/activitystreams#Public"],
                "name":"Funkwhale track"}}}}
            ]}}"#
            ),
        ),
        _ => panic!("unexpected target {target}"),
    });
    let source = ActivityPub::new(&base).unwrap();
    let posts = source
        .try_fetch_last_posts_by_user(&format!("{base}users/alice"), 2)
        .unwrap();

    assert_eq!(posts.len(), 2);
    assert_eq!(posts[0].content.as_deref(), Some("BookWyrm post"));
    assert_eq!(posts[1].content.as_deref(), Some("Funkwhale track"));
}
