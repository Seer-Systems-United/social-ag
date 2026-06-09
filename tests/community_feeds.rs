#[path = "support/feed_source.rs"]
mod feed_support;

use social_ag::{ProductHunt, SocialSource, SourceQuirk, SteamCommunity};

#[test]
fn constructs_community_feeds() {
    let product_hunt = ProductHunt::new().unwrap();
    assert_eq!(product_hunt.definition().name, "ProductHunt");
    assert!(
        product_hunt
            .definition()
            .quirks
            .contains(&SourceQuirk::CommunityAsUser)
    );
    assert_eq!(
        product_hunt
            .lookup_user_by_id("producthunt")
            .unwrap()
            .username,
        "producthunt"
    );

    feed_support::assert_source(
        SteamCommunity::new("testuser").unwrap(),
        "SteamCommunity",
        "steamcommunity.com/groups/testuser/rss",
    );
}
