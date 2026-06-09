mod support;

use social_ag::{FeedSource, SocialSource, User};
use support::{Response, server};

fn publisher() -> User {
    User {
        id: "example".into(),
        username: "example".into(),
        display_name: Some("Example".into()),
        profile_url: "https://example.com/".into(),
    }
}

#[test]
fn parses_rss() {
    let base = server(1, |_, _| {
        Response::new(
            "application/xml",
            r#"<?xml version="1.0"?>
            <rss version="2.0"><channel><title>Example</title>
            <link>https://example.com</link><description>Feed</description>
            <item><title>Update</title><link>https://example.com/update</link>
            <guid>update-1</guid><pubDate>Mon, 08 Jun 2026 20:15:30 +0000</pubDate>
            <description><![CDATA[<p>Feed body</p>]]></description>
            </item></channel></rss>"#,
        )
    });
    let source = FeedSource::new(format!("{base}feed.xml"), publisher()).unwrap();
    let post = source.fetch_latest_post_by_user("example").unwrap();

    assert_eq!(post.content.as_deref(), Some("Feed body"));
}

#[test]
fn parses_atom() {
    let base = server(1, |_, _| {
        Response::new(
            "application/xml",
            r#"<?xml version="1.0" encoding="utf-8"?>
            <feed xmlns="http://www.w3.org/2005/Atom"><title>Example</title>
            <id>https://example.com/</id><updated>2026-06-08T20:15:30Z</updated>
            <entry><title>Update</title><id>urn:example:update-1</id>
            <updated>2026-06-08T20:15:30Z</updated>
            <link rel="alternate" href="https://example.com/update"/>
            <content type="html">&lt;p&gt;Atom body&lt;/p&gt;</content>
            </entry></feed>"#,
        )
    });
    let source = FeedSource::new(format!("{base}feed.atom"), publisher()).unwrap();
    let post = source.fetch_latest_post_by_user("example").unwrap();

    assert_eq!(post.content.as_deref(), Some("Atom body"));
}

#[test]
fn rejects_non_http_urls() {
    assert!(FeedSource::new("file:///tmp/feed.xml", publisher()).is_err());
}
