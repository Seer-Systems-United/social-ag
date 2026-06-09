mod support;

use social_ag::{Farcaster, ParseType, SocialSource, SourceQuirk};
use support::{Response, server};

const PROFILE: &str = r#"{"messages":[
  {"data":{"fid":2,"userDataBody":{"type":"USER_DATA_TYPE_USERNAME","value":"v"}}},
  {"data":{"fid":2,"userDataBody":{"type":"USER_DATA_TYPE_DISPLAY","value":"Varun"}}}
]}"#;

#[test]
fn parses_profiles_and_paginated_casts() {
    let base = server(4, |target, _| match target {
        "/userDataByFid?fid=2" => Response::new("application/json", PROFILE),
        "/castsByFid?fid=2&pageSize=2&reverse=true" => Response::new(
            "application/json",
            page(Some("next"), cast("0xnew", 100, "Newer", None)),
        ),
        "/castsByFid?fid=2&pageSize=2&reverse=true&pageToken=next" => Response::new(
            "application/json",
            page(Some("unused"), cast("0xold", 50, "Older", Some("0xparent"))),
        ),
        _ => panic!("unexpected target: {target}"),
    });
    let source = Farcaster::new("2").unwrap().with_hub_url(base).unwrap();
    let user = source.try_lookup_user_by_username("@v").unwrap().unwrap();
    let posts = source.try_fetch_last_posts_by_user(&user.id, 2).unwrap();

    assert_eq!(source.parse_type(), ParseType::PublicJson);
    assert!(
        source
            .definition()
            .quirks
            .contains(&SourceQuirk::UndocumentedPublicEndpoint)
    );
    assert_eq!(user.display_name.as_deref(), Some("Varun"));
    assert_eq!(posts[0].id, "0xnew");
    assert_eq!(posts[1].in_reply_to_id.as_deref(), Some("0xparent"));
}

fn page(token: Option<&str>, message: String) -> String {
    let token = token.map_or("null".into(), |value| format!(r#""{value}""#));
    format!(r#"{{"messages":[{message}],"nextPageToken":{token}}}"#)
}

fn cast(hash: &str, timestamp: i64, text: &str, parent: Option<&str>) -> String {
    let parent = parent.map_or("null".into(), |value| format!(r#"{{"hash":"{value}"}}"#));
    format!(
        r#"{{"hash":"{hash}","data":{{"timestamp":{timestamp},"castAddBody":{{"text":"{text}","parentCastId":{parent}}}}}}}"#
    )
}
