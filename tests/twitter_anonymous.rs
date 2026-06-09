mod support;

use social_ag::{SocialSource, Twitter};
use support::{Response, server};

const TIMELINE: &str = r#"<!doctype html><html><body>
<script id="__NEXT_DATA__" type="application/json">{
  "props":{"pageProps":{
    "contextProvider":{"hasResults":true},
    "timeline":{"entries":[
      {"content":{"tweet":{
        "id_str":"1","created_at":"Sun Jun 08 19:00:00 +0000 2025",
        "full_text":"Older &amp; first &#x27;post&#39;","text":"Older",
        "permalink":"/alice/status/1",
        "user":{"name":"Alice Example","screen_name":"alice"},
        "in_reply_to_status_id_str":null
      }}},
      {"content":{"tweet":{
        "id_str":"2","created_at":"Mon Jun 09 20:00:00 +0000 2025",
        "full_text":"Newer post","text":"Newer",
        "permalink":"/alice/status/2",
        "user":{"name":"Alice Example","screen_name":"alice"},
        "in_reply_to_status_id_str":"1"
      }}}
    ]}
  }}
}</script></body></html>"#;

fn source_at(requests: usize, body: &'static str) -> Twitter {
    let base = server(requests, move |target, _| {
        assert_eq!(target, "/alice");
        Response::new("text/html", body)
    });
    Twitter::new().unwrap().with_syndication_url(base).unwrap()
}

#[test]
fn reads_public_profile_and_sorts_posts() {
    let source = source_at(2, TIMELINE);
    let user = source
        .try_lookup_user_by_username("@alice")
        .unwrap()
        .unwrap();
    let posts = source.try_fetch_last_posts_by_user(&user.id, 2).unwrap();

    assert_eq!(user.id, "alice");
    assert_eq!(user.display_name.as_deref(), Some("Alice Example"));
    assert_eq!(posts[0].id, "2");
    assert_eq!(posts[0].in_reply_to_id.as_deref(), Some("1"));
    assert_eq!(posts[1].content.as_deref(), Some("Older & first 'post'"));
}

#[test]
fn reports_missing_public_profiles() {
    const MISSING: &str = r#"<script id="__NEXT_DATA__" type="application/json">
      {"props":{"pageProps":{"contextProvider":{"hasResults":false},
      "timeline":{"entries":[]}}}}</script>"#;
    let source = source_at(1, MISSING);

    assert!(
        source
            .try_lookup_user_by_username("alice")
            .unwrap()
            .is_none()
    );
}
