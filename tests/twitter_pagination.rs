mod support;

use social_ag::{SocialSource, Twitter};
use support::{Response, server};

#[test]
fn follows_x_pagination_tokens() {
    let base = server(3, |target, _| {
        if target.starts_with("/2/users/by/username/alice?") {
            return Response::new(
                "application/json",
                r#"{"data":{"id":"42","name":"Alice","username":"alice"}}"#,
            );
        }
        if target.contains("pagination_token=next") {
            return Response::new(
                "application/json",
                r#"{"data":[
                    {"id":"6","text":"Six","created_at":"2026-06-08T15:00:00Z"}
                ]}"#,
            );
        }
        Response::new(
            "application/json",
            r#"{"data":[
                {"id":"1","text":"One","created_at":"2026-06-08T20:00:00Z"},
                {"id":"2","text":"Two","created_at":"2026-06-08T19:00:00Z"},
                {"id":"3","text":"Three","created_at":"2026-06-08T18:00:00Z"},
                {"id":"4","text":"Four","created_at":"2026-06-08T17:00:00Z"},
                {"id":"5","text":"Five","created_at":"2026-06-08T16:00:00Z"}
            ],"meta":{"next_token":"next"}}"#,
        )
    });
    let source = Twitter::new()
        .unwrap()
        .with_bearer_token("token")
        .with_api_url(&base)
        .unwrap();
    let posts = source.try_fetch_last_posts_by_user("alice", 6).unwrap();

    assert_eq!(posts.len(), 6);
    assert_eq!(posts.last().unwrap().id, "6");
}
