mod support;

use social_ag::{Bluesky, SocialSource, TransportConfig};
use support::{Response, server};

const ALICE_DID: &str = "did:plc:ewvi7nxzyoun6zhxrhs64oiz";
const OTHER_DID: &str = "did:plc:z72i7hdynmk6r22z27h6tvur";
const CID: &str = "bafyreia6xwz3im4ww5pajv4q5nby4z7u4wzj2vmb3g3xv2qqx6j5q2z3ae";

#[test]
fn uses_generated_schemas_and_filters_reposts() {
    let base = server(3, |target, _| {
        if target.starts_with("/xrpc/app.bsky.actor.getProfile") {
            return Response::new(
                "application/json",
                format!(
                    r#"{{"did":"{ALICE_DID}","handle":"alice.example","displayName":"Alice"}}"#
                ),
            );
        }
        assert!(target.starts_with("/xrpc/app.bsky.feed.getAuthorFeed"));
        Response::new(
            "application/json",
            format!(
                r#"{{
                "feed":[
                    {{"post":{{
                        "uri":"at://{OTHER_DID}/app.bsky.feed.post/repost","cid":"{CID}",
                        "author":{{"did":"{OTHER_DID}","handle":"other.example"}},
                        "record":{{"text":"Repost","createdAt":"2026-06-08T19:00:00Z"}},
                        "indexedAt":"2026-06-08T19:00:01Z"
                    }}}},
                    {{"post":{{
                        "uri":"at://{ALICE_DID}/app.bsky.feed.post/3abc","cid":"{CID}",
                        "author":{{"did":"{ALICE_DID}","handle":"alice.example","displayName":"Alice"}},
                        "record":{{"text":"Hello AT Protocol","createdAt":"2026-06-08T20:00:00Z"}},
                        "indexedAt":"2026-06-08T20:00:01Z"
                    }}}}
                ]
            }}"#
            ),
        )
    });
    let source =
        Bluesky::new_with_urls(&base, "https://bsky.app/", TransportConfig::default()).unwrap();
    let user = source.lookup_user_by_username("alice.example").unwrap();
    let post = source.fetch_latest_post_by_user("alice.example").unwrap();

    assert_eq!(post.publisher_user.id, user.id);
    assert_eq!(post.content.as_deref(), Some("Hello AT Protocol"));
}
