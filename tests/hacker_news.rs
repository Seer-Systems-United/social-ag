mod support;

use social_ag::{Capability, HackerNews, ParseType, SocialSource};
use support::{Response, server};

fn source_at(base: &str) -> HackerNews {
    HackerNews::new()
        .unwrap()
        .with_base_url(base)
        .unwrap()
}

#[test]
fn looks_up_user_by_username() {
    let base = server(1, |target, _| {
        assert_eq!(target, "/v0/user/pg.json");
        Response::new(
            "application/json",
            r#"{"id":"pg","created":1160418091,"about":"Founder of YC"}"#,
        )
    });
    let source = source_at(&base);
    let user = source.try_lookup_user_by_username("pg").unwrap().unwrap();

    assert_eq!(user.id, "pg");
    assert_eq!(user.username, "pg");
    assert!(user.profile_url.contains("news.ycombinator.com/user?id=pg"));
}

#[test]
fn looks_up_user_by_id() {
    let base = server(1, |target, _| {
        assert_eq!(target, "/v0/user/patio11.json");
        Response::new("application/json", r#"{"id":"patio11"}"#)
    });
    let source = source_at(&base);
    let user = source.try_lookup_user_by_id("patio11").unwrap().unwrap();

    assert_eq!(user.id, "patio11");
}

#[test]
fn returns_none_for_missing_user() {
    let base = server(1, |_, _| Response::error(404, "Not Found"));
    let source = source_at(&base);
    assert!(source.try_lookup_user_by_username("noone").unwrap().is_none());
}

#[test]
fn display_name_returns_none() {
    let source = HackerNews::default();
    assert!(source.try_lookup_user_by_display_name("anything").unwrap().is_none());
}

#[test]
fn fetch_latest_post_returns_story() {
    let base = server(2, |target, _| {
        if target == "/v0/user/test.json" {
            return Response::new("application/json", r#"{"submitted":[42,99]}"#);
        }
        if target == "/v0/item/42.json" {
            return Response::new(
                "application/json",
                r#"{"id":42,"type":"story","title":"Hello","url":"https://example.com","time":1710000000,"by":"test"}"#,
            );
        }
        panic!("unexpected target: {target}");
    });
    let source = source_at(&base);
    let post = source.try_fetch_latest_post_by_user("test").unwrap().unwrap();

    assert_eq!(post.title.as_deref(), Some("Hello"));
    assert_eq!(post.url, "https://example.com");
    assert_eq!(post.publisher_user.username, "test");
}

#[test]
fn fetch_last_posts_filters_non_stories() {
    let base = server(4, |target, _| {
        match target {
            "/v0/user/test.json" => Response::new("application/json", r#"{"submitted":[1,2,3]}"#),
            "/v0/item/1.json" => Response::new("application/json", r#"{"id":1,"type":"comment"}"#),
            "/v0/item/2.json" => Response::new("application/json", r#"{"id":2,"type":"story","title":"Story","time":1710000000,"by":"test","url":"https://a.com"}"#),
            "/v0/item/3.json" => Response::new("application/json", r#"{"id":3,"type":"poll","title":"Poll","time":1710000001,"by":"test","url":"https://b.com"}"#),
            _ => panic!("unexpected target: {target}"),
        }
    });
    let source = source_at(&base);
    let posts = source.try_fetch_last_posts_by_user("test", 3).unwrap();

    assert_eq!(posts.len(), 2);
    assert_eq!(posts[0].title.as_deref(), Some("Story"));
    assert_eq!(posts[1].title.as_deref(), Some("Poll"));
}

#[test]
fn fetch_posts_with_no_submissions_returns_empty() {
    let base = server(1, |target, _| {
        assert_eq!(target, "/v0/user/test.json");
        Response::new("application/json", r#"{"submitted":null}"#)
    });
    let source = source_at(&base);
    let posts = source.try_fetch_last_posts_by_user("test", 5).unwrap();

    assert!(posts.is_empty());
}

#[test]
fn empty_count_returns_empty() {
    let source = HackerNews::default();
    let posts = source.try_fetch_last_posts_by_user("test", 0).unwrap();
    assert!(posts.is_empty());
}

#[test]
fn fetches_self_hosted_item_if_no_url() {
    let base = server(2, |target, _| {
        match target {
            "/v0/user/t.json" => Response::new("application/json", r#"{"submitted":[100]}"#),
            "/v0/item/100.json" => Response::new(
                "application/json",
                r#"{"id":100,"type":"story","title":"Ask HN","time":1710000000,"by":"t"}"#,
            ),
            _ => panic!("unexpected target: {target}"),
        }
    });
    let source = source_at(&base);
    let post = source.try_fetch_latest_post_by_user("t").unwrap().unwrap();

    assert!(post.url.contains("news.ycombinator.com/item?id=100"));
}

#[test]
fn definition_is_correct() {
    let source = HackerNews::default();
    let def = source.definition();

    assert_eq!(def.name, "HackerNews");
    assert_eq!(def.protocol, ParseType::Feed);
    assert!(def.capabilities.contains(&Capability::FetchUserPosts));
    assert!(def.capabilities.contains(&Capability::LookupUserByUsername));
}

#[test]
fn bad_base_url_errors() {
    let result = HackerNews::new().unwrap().with_base_url("not-a-url");
    assert!(result.is_err());
}

#[test]
fn lookup_user_with_bad_url_errors() {
    let source = HackerNews::new().unwrap()
        .with_base_url("http://127.0.0.1:1")
        .unwrap();
    let err = source.try_lookup_user_by_username("pg").unwrap_err();
    assert!(err.to_string().contains("error sending request"), "expected connection error, got: {err}");
}

#[test]
fn lookup_user_by_id_with_bad_url_errors() {
    let source = HackerNews::new().unwrap()
        .with_base_url("http://127.0.0.1:1")
        .unwrap();
    let err = source.try_lookup_user_by_id("pg").unwrap_err();
    assert!(err.to_string().contains("error sending request"), "expected connection error, got: {err}");
}


