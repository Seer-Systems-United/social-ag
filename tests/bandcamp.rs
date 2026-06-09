mod support;

use social_ag::{Bandcamp, ParseType, SocialSource};
use support::{Response, server};

const CATALOG: &str = r#"<html><body>
<p id="band-name-location"><span class="title">Example Artist</span></p>
<ol><li class="music-grid-item" data-item-id="album-42">
  <a href="/album/example"><p class="title">Example Album</p></a>
</li></ol>
</body></html>"#;

const RELEASE: &str = r#"<html><head>
<script type="application/ld+json">{
  "@id":"https://example.test/album/example",
  "name":"Example Album",
  "datePublished":"09 Jun 2026 12:30:00 GMT",
  "description":"An example release"
}</script>
</head></html>"#;

#[test]
fn parses_catalog_and_release_json_ld() {
    let base = server(3, |target, _| match target {
        "/music" => Response::new("text/html", CATALOG),
        "/album/example" => Response::new("text/html", RELEASE),
        _ => panic!("unexpected target: {target}"),
    });
    let source = Bandcamp::new("example")
        .unwrap()
        .with_artist_url(format!("{base}music"))
        .unwrap();
    let user = source
        .try_lookup_user_by_username("example")
        .unwrap()
        .unwrap();
    let post = source
        .try_fetch_latest_post_by_user(&user.id)
        .unwrap()
        .unwrap();

    assert_eq!(source.parse_type(), ParseType::PublicHtml);
    assert_eq!(user.display_name.as_deref(), Some("Example Artist"));
    assert_eq!(post.id, "album-42");
    assert_eq!(post.content.as_deref(), Some("An example release"));
}
