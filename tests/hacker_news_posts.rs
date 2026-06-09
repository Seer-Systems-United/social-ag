mod support;

use social_ag::{HackerNews, SocialSource};
use support::{Response, server};

fn source_at(base: &str) -> HackerNews {
    HackerNews::new().unwrap().with_base_url(base).unwrap()
}

#[test]
fn fetches_and_filters_posts() {
    let base = server(4, |target, _| match target {
        "/v0/user/test.json" => {
            Response::new("application/json", r#"{"id":"test","submitted":[1,2,3]}"#)
        }
        "/v0/item/1.json" => Response::new("application/json", r#"{"id":1,"type":"comment"}"#),
        "/v0/item/2.json" => Response::new(
            "application/json",
            r#"{"id":2,"type":"story","title":"Story","time":1710000000,"url":"https://a.com"}"#,
        ),
        "/v0/item/3.json" => Response::new(
            "application/json",
            r#"{"id":3,"type":"poll","title":"Poll","time":1710000001}"#,
        ),
        _ => panic!("unexpected target: {target}"),
    });
    let posts = source_at(&base)
        .try_fetch_last_posts_by_user("test", 3)
        .unwrap();

    assert_eq!(posts.len(), 2);
    assert_eq!(posts[0].title.as_deref(), Some("Story"));
    assert!(posts[1].url.ends_with("item?id=3"));
}

#[test]
fn handles_empty_posts_and_counts() {
    let base = server(1, |_, _| {
        Response::new("application/json", r#"{"id":"test","submitted":null}"#)
    });
    let source = source_at(&base);

    assert!(
        source
            .try_fetch_last_posts_by_user("test", 5)
            .unwrap()
            .is_empty()
    );
    assert!(
        HackerNews::default()
            .try_fetch_last_posts_by_user("test", 0)
            .unwrap()
            .is_empty()
    );
}
