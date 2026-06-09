#[path = "support/feed_source.rs"]
mod feed_support;

use feed_support::assert_source;
use social_ag::{
    ArchiveOfOurOwn, BitChute, DevTo, DeviantArt, Hashnode, Letterboxd, Medium, Mixcloud, Odysee,
    Reddit, SocialSource, Substack, Tumblr, Vimeo,
};

#[test]
fn constructs_user_feed_sources() {
    assert_source(
        Medium::new("testuser").unwrap(),
        "Medium",
        "medium.com/feed/@testuser",
    );
    assert_source(
        Substack::new("testuser").unwrap(),
        "Substack",
        "testuser.substack.com/feed",
    );
    assert_source(
        DevTo::new("testuser").unwrap(),
        "DevTo",
        "dev.to/feed/testuser",
    );
    assert_source(
        Hashnode::new("testuser").unwrap(),
        "Hashnode",
        "testuser.hashnode.dev/rss",
    );
    assert_source(
        Tumblr::new("testuser").unwrap(),
        "Tumblr",
        "testuser.tumblr.com/rss",
    );
    assert_source(
        Letterboxd::new("testuser").unwrap(),
        "Letterboxd",
        "letterboxd.com/testuser/rss/",
    );
    assert_source(
        Reddit::new("testuser").unwrap(),
        "Reddit",
        "reddit.com/user/testuser/.rss",
    );
    assert_source(
        DeviantArt::new("testuser").unwrap(),
        "DeviantArt",
        "backend.deviantart.com/rss.xml",
    );
    assert_source(
        Mixcloud::new("testuser").unwrap(),
        "Mixcloud",
        "mixcloud.com/testuser/feed/",
    );
    assert_source(
        Vimeo::new("testuser").unwrap(),
        "Vimeo",
        "vimeo.com/testuser/videos/rss",
    );
    assert_source(
        ArchiveOfOurOwn::new("testuser").unwrap(),
        "ArchiveOfOurOwn",
        "archiveofourown.org/users/testuser/feed",
    );
    assert_source(
        BitChute::new("testuser").unwrap(),
        "BitChute",
        "bitchute.com/feed/testuser",
    );
    assert_source(
        Odysee::new("testuser").unwrap(),
        "Odysee",
        "odysee.com/$/rss/@testuser",
    );
}

#[test]
fn normalizes_and_rejects_other_users() {
    let source = Medium::new("@testuser").unwrap();
    assert_eq!(
        source.lookup_user_by_username("testuser").unwrap().id,
        "testuser"
    );
    assert!(source.lookup_user_by_username("otheruser").is_none());
    assert!(source.fetch_latest_post_by_user("wronguser").is_none());
}
