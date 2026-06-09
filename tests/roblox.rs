mod support;

use social_ag::{ParseType, Roblox, SocialSource, SourceQuirk};
use support::{Response, server};

const USER: &str = r#"{"id":1,"name":"Roblox","displayName":"Roblox"}"#;

#[test]
fn parses_users_and_paginated_games() {
    let base = server(4, |target, base| match target {
        "/users/1" => Response::new("application/json", USER),
        "/games/1/games?accessFilter=Public&limit=10&sortOrder=Desc" => Response::new(
            "application/json",
            page(Some("next"), game(10, 100, "2026-06-09T10:00:00Z"), base),
        ),
        "/games/1/games?accessFilter=Public&limit=10&sortOrder=Desc&cursor=next" => Response::new(
            "application/json",
            page(None, game(9, 90, "2026-06-08T10:00:00Z"), base),
        ),
        _ => panic!("unexpected target: {target}"),
    });
    let source = Roblox::new("1")
        .unwrap()
        .with_api_urls(format!("{base}users/"), format!("{base}games/"))
        .unwrap();
    let user = source
        .try_lookup_user_by_username("roblox")
        .unwrap()
        .unwrap();
    let posts = source.try_fetch_last_posts_by_user(&user.id, 2).unwrap();

    assert_eq!(source.parse_type(), ParseType::PublicJson);
    assert!(
        source
            .definition()
            .quirks
            .contains(&SourceQuirk::GamesAsPosts)
    );
    assert_eq!(posts[0].id, "10");
    assert_eq!(posts[1].url, "https://www.roblox.com/games/90");
}

fn page(cursor: Option<&str>, game: String, _: &str) -> String {
    format!(
        r#"{{"nextPageCursor":{},"data":[{game}]}}"#,
        cursor.map_or("null".into(), |value| format!(r#""{value}""#))
    )
}

fn game(id: u64, place: u64, updated: &str) -> String {
    format!(
        r#"{{"id":{id},"name":"Game {id}","description":"Text","updated":"{updated}","rootPlace":{{"id":{place}}}}}"#
    )
}
