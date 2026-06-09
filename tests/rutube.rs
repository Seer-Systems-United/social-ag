mod support;

use social_ag::{ParseType, Rutube, SocialSource};
use support::{Response, server};

const PAGE: &str = r#"{
  "next": null,
  "results": [
    {
      "id": "newer",
      "title": "Newer video",
      "description": "Description",
      "publication_ts": "2026-06-09T20:37:48",
      "video_url": "https://rutube.ru/video/newer/",
      "author": {"id": 42, "name": "Example Channel"}
    },
    {
      "id": "older",
      "title": "Older video",
      "description": "",
      "publication_ts": "2026-06-08T20:37:48",
      "video_url": "https://rutube.ru/video/older/",
      "author": {"id": 42, "name": "Example Channel"}
    }
  ]
}"#;

#[test]
fn parses_public_channel_api() {
    let base = server(2, |target, _| {
        assert_eq!(target, "/42/?format=json");
        Response::new("application/json", PAGE)
    });
    let source = Rutube::new("42").unwrap().with_api_url(base).unwrap();
    let user = source.try_lookup_user_by_id("42").unwrap().unwrap();
    let posts = source.try_fetch_last_posts_by_user(&user.id, 2).unwrap();

    assert_eq!(source.parse_type(), ParseType::PublicJson);
    assert_eq!(user.display_name.as_deref(), Some("Example Channel"));
    assert_eq!(posts[0].id, "newer");
    assert_eq!(posts[1].content, None);
}
