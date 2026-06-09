mod support;

use social_ag::{FourChan, ParseType, SocialSource, SourceQuirk};
use support::{Response, server};

#[test]
fn parses_board_catalogs() {
    let base = server(1, |target, _| {
        assert_eq!(target, "/g/catalog.json");
        Response::new(
            "application/json",
            r#"[{"threads":[
              {"no":1,"time":1781000000,"sub":"Older","com":"First &amp; post"},
              {"no":2,"time":1781100000,"sub":"Newer","com":"Second post"}
            ]}]"#,
        )
    });
    let source = FourChan::new("g").unwrap().with_api_url(base).unwrap();
    let user = source.try_lookup_user_by_username("g").unwrap().unwrap();
    let posts = source.try_fetch_last_posts_by_user(&user.id, 2).unwrap();

    assert_eq!(source.parse_type(), ParseType::PublicJson);
    assert!(
        source
            .definition()
            .quirks
            .contains(&SourceQuirk::BoardAsUser)
    );
    assert_eq!(user.display_name.as_deref(), Some("/g/"));
    assert_eq!(posts[0].id, "2");
    assert_eq!(posts[1].content.as_deref(), Some("First & post"));
}
