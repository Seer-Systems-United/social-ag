mod support;

use social_ag::{ParseType, SocialSource, Telegram};
use support::{Response, server};

const PAGE: &str = r#"<html><body>
<div class="tgme_channel_info_header_title">Example Channel</div>
<div class="tgme_widget_message" data-post="example/1">
  <div class="tgme_widget_message_text"><b>Older</b> message</div>
  <a class="tgme_widget_message_date" href="https://t.me/example/1">
    <time datetime="2026-06-08T10:00:00+00:00">10:00</time>
  </a>
</div>
<div class="tgme_widget_message" data-post="example/2">
  <div class="tgme_widget_message_text">Newer message</div>
  <a class="tgme_widget_message_date" href="https://t.me/example/2">
    <time datetime="2026-06-09T10:00:00+00:00">10:00</time>
  </a>
</div>
</body></html>"#;

#[test]
fn parses_public_channel_pages() {
    let base = server(1, |target, _| {
        assert_eq!(target, "/channel");
        Response::new("text/html", PAGE)
    });
    let source = Telegram::new("example")
        .unwrap()
        .with_fetch_url(format!("{base}channel"))
        .unwrap();
    let user = source
        .try_lookup_user_by_username("@example")
        .unwrap()
        .unwrap();
    let posts = source.try_fetch_last_posts_by_user(&user.id, 2).unwrap();

    assert_eq!(source.parse_type(), ParseType::PublicHtml);
    assert_eq!(user.display_name.as_deref(), Some("Example Channel"));
    assert_eq!(posts[0].id, "2");
    assert_eq!(posts[1].content.as_deref(), Some("Older message"));
}
