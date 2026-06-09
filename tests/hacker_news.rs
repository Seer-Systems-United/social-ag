mod support;

use social_ag::{Capability, HackerNews, ParseType, SocialSource};
use support::{Response, server};

fn source_at(base: &str) -> HackerNews {
    HackerNews::new().unwrap().with_base_url(base).unwrap()
}

#[test]
fn looks_up_users() {
    let base = server(2, |target, _| match target {
        "/v0/user/pg.json" => Response::new("application/json", r#"{"id":"pg"}"#),
        "/v0/user/patio11.json" => Response::new("application/json", r#"{"id":"patio11"}"#),
        _ => panic!("unexpected target: {target}"),
    });
    let source = source_at(&base);
    let pg = source.try_lookup_user_by_username("pg").unwrap().unwrap();
    let patio = source.try_lookup_user_by_id("patio11").unwrap().unwrap();

    assert_eq!(pg.username, "pg");
    assert!(pg.profile_url.ends_with("user?id=pg"));
    assert_eq!(patio.id, "patio11");
}

#[test]
fn handles_missing_and_unsupported_lookups() {
    let base = server(1, |_, _| Response::error(404, "Not Found"));
    let source = source_at(&base);

    assert!(
        source
            .try_lookup_user_by_username("noone")
            .unwrap()
            .is_none()
    );
    assert!(
        source
            .try_lookup_user_by_display_name("anything")
            .unwrap()
            .is_none()
    );
}

#[test]
fn definition_is_correct() {
    let definition = HackerNews::default().definition();

    assert_eq!(definition.name, "HackerNews");
    assert_eq!(definition.protocol, ParseType::Feed);
    assert!(
        definition
            .capabilities
            .contains(&Capability::FetchUserPosts)
    );
}
