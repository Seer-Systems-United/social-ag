mod support;

use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use social_ag::{Lens, ParseType, SocialSource};
use support::{Response, server};

const ACCOUNT: &str = r#"{"data":{"account":{
  "address":"0x1234",
  "username":{"localName":"lens"},
  "metadata":{"name":"Lens"}
}}}"#;

#[test]
fn parses_accounts_and_paginated_posts() {
    let requests = Arc::new(AtomicUsize::new(0));
    let request_index = requests.clone();
    let base = server(3, move |target, _| {
        assert_eq!(target, "/graphql");
        match request_index.fetch_add(1, Ordering::SeqCst) {
            0 => Response::new("application/json", ACCOUNT),
            1 => Response::new(
                "application/json",
                post_page(Some("next"), post("1", "newer", "2026-06-09T10:00:00Z")),
            ),
            2 => Response::new(
                "application/json",
                post_page(None, post("2", "older", "2026-06-08T10:00:00Z")),
            ),
            index => panic!("unexpected request: {index}"),
        }
    });
    let source = Lens::new("lens")
        .unwrap()
        .with_api_url(format!("{base}graphql"))
        .unwrap();
    let user = source
        .try_lookup_user_by_username("lens/lens")
        .unwrap()
        .unwrap();
    let posts = source.try_fetch_last_posts_by_user(&user.id, 2).unwrap();

    assert_eq!(source.parse_type(), ParseType::GraphQl);
    assert_eq!(user.display_name.as_deref(), Some("Lens"));
    assert_eq!(posts[0].id, "1");
    assert_eq!(posts[1].url, "https://hey.xyz/posts/older");
}

fn post_page(cursor: Option<&str>, item: String) -> String {
    let cursor = cursor.map_or("null".into(), |value| format!(r#""{value}""#));
    format!(r#"{{"data":{{"posts":{{"items":[{item}],"pageInfo":{{"next":{cursor}}}}}}}}}"#)
}

fn post(id: &str, slug: &str, timestamp: &str) -> String {
    format!(
        r#"{{"id":"{id}","slug":"{slug}","timestamp":"{timestamp}","metadata":{{"title":null,"content":"Post {id}"}}}}"#
    )
}
