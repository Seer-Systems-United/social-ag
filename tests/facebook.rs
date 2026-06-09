mod support;

use social_ag::{Authentication, Facebook, ParseType, SocialSource, SourceError};
use support::{Response, server};

fn source_at(base: &str) -> Facebook {
    Facebook::new()
        .with_access_token("page-token")
        .with_api_url(base)
        .unwrap()
}

#[test]
fn requires_graph_api_authentication() {
    let source = Facebook::default();
    let definition = source.definition();

    assert_eq!(definition.protocol, ParseType::Facebook);
    assert_eq!(definition.authentication, Authentication::RequiredBearer);
    assert!(matches!(
        source.try_lookup_user_by_id("42"),
        Err(SourceError::AuthenticationRequired)
    ));
}

#[test]
fn reads_pages_and_paginated_posts() {
    let base = server(3, |target, base| {
        if target.starts_with("/news?") {
            return Response::new(
                "application/json",
                r#"{"id":"42","name":"News Page","username":"news",
                "link":"https://www.facebook.com/news"}"#,
            );
        }
        if target.contains("after=next-page") {
            return Response::new(
                "application/json",
                format!(
                    r#"{{"data":[{{"id":"42_2","message":"Second",
                    "created_time":"2026-06-08T19:00:00+0000",
                    "permalink_url":"{base}posts/2"}}]}}"#
                ),
            );
        }
        assert!(target.starts_with("/42/published_posts?"));
        Response::new(
            "application/json",
            format!(
                r#"{{"data":[{{"id":"42_1","message":"First",
                "created_time":"2026-06-08T20:00:00Z",
                "permalink_url":"{base}posts/1"}}],
                "paging":{{"cursors":{{"after":"next-page"}}}}}}"#
            ),
        )
    });
    let source = source_at(&base);
    let posts = source.try_fetch_last_posts_by_user("news", 2).unwrap();

    assert_eq!(posts.len(), 2);
    assert_eq!(posts[0].publisher_user.username, "news");
    assert_eq!(posts[1].content.as_deref(), Some("Second"));
}
