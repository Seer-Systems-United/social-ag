mod support;

use social_ag::{Authentication, ParseType, SocialSource, SourceQuirk, Twitter};
use support::{Response, server};

fn source_at(base: &str) -> Twitter {
    Twitter::new()
        .unwrap()
        .with_bearer_token("app-token")
        .with_api_url(base)
        .unwrap()
}

#[test]
fn describes_optional_x_api_authentication() {
    let source = Twitter::default();
    let definition = source.definition();

    assert_eq!(definition.protocol, ParseType::Twitter);
    assert_eq!(definition.authentication, Authentication::OptionalBearer);
    assert!(
        definition
            .quirks
            .contains(&SourceQuirk::UndocumentedPublicEndpoint)
    );
}

#[test]
fn reads_users_and_posts() {
    let base = server(3, |target, _| {
        if target.starts_with("/2/users/by/username/alice?") {
            return Response::new(
                "application/json",
                r#"{"data":{"id":"42","name":"Alice","username":"alice"}}"#,
            );
        }
        assert!(target.starts_with("/2/users/42/tweets?"));
        Response::new(
            "application/json",
            r#"{"data":[
                {"id":"2","text":"Reply","created_at":"2026-06-08T20:00:00Z",
                 "referenced_tweets":[{"type":"replied_to","id":"1"}]},
                {"id":"1","text":"Original","created_at":"2026-06-08T19:00:00Z"}
            ]}"#,
        )
    });
    let source = source_at(&base);
    let user = source
        .try_lookup_user_by_username("@alice")
        .unwrap()
        .unwrap();
    let posts = source.try_fetch_last_posts_by_user("alice", 2).unwrap();

    assert_eq!(user.id, "42");
    assert_eq!(user.display_name.as_deref(), Some("Alice"));
    assert_eq!(posts.len(), 2);
    assert_eq!(posts[0].in_reply_to_id.as_deref(), Some("1"));
    assert_eq!(posts[1].url, "https://x.com/alice/status/1");
}
