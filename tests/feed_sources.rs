use social_ag::{
    ArchiveOfOurOwn, BitChute, Capability, DevTo, DeviantArt, Hashnode, Letterboxd, Medium,
    Mixcloud, Odysee, ParseType, Reddit, SocialSource, SourceDefinition, Substack, Tumblr, Vimeo,
};

fn assert_source_definition(source: &impl SocialSource, name: &str) {
    let def: SourceDefinition = source.definition();
    assert_eq!(def.name, name);
    assert_eq!(def.protocol, ParseType::Feed);
    assert!(def.capabilities.contains(&Capability::FetchUserPosts));
}

fn assert_lookup(source: &impl SocialSource, username: &str) {
    let user = source.lookup_user_by_username(username).unwrap();
    assert_eq!(user.username, username);
    assert_eq!(user.id, username);

    let by_id = source.lookup_user_by_id(username).unwrap();
    assert_eq!(by_id.username, username);
}

fn assert_no_match(source: &impl SocialSource, username: &str) {
    assert!(source.lookup_user_by_username(username).is_none());
    assert!(source.lookup_user_by_id(username).is_none());
    assert!(source.lookup_user_by_display_name("nonexistent").is_none());
}

#[test]
fn medium_constructs_correct_feed_url() {
    let source = Medium::new("testuser").unwrap();
    assert!(source.feed_url().as_str().contains("medium.com/feed/@testuser"));
    assert_source_definition(&source, "Medium");
    assert_lookup(&source, "testuser");
    assert_no_match(&source, "otheruser");
}

#[test]
fn medium_strips_at_sign() {
    let source = Medium::new("@testuser").unwrap();
    assert!(source.feed_url().as_str().contains("medium.com/feed/@testuser"));
    assert_lookup(&source, "testuser");
}

#[test]
fn substack_constructs_correct_feed_url() {
    let source = Substack::new("testuser").unwrap();
    assert!(source.feed_url().as_str().contains("testuser.substack.com/feed"));
    assert_source_definition(&source, "Substack");
    assert_lookup(&source, "testuser");
    assert_no_match(&source, "otheruser");
}

#[test]
fn dev_to_constructs_correct_feed_url() {
    let source = DevTo::new("testuser").unwrap();
    assert!(source.feed_url().as_str().contains("dev.to/feed/testuser"));
    assert_source_definition(&source, "DevTo");
    assert_lookup(&source, "testuser");
    assert_no_match(&source, "otheruser");
}

#[test]
fn hashnode_constructs_correct_feed_url() {
    let source = Hashnode::new("testuser").unwrap();
    assert!(source.feed_url().as_str().contains("testuser.hashnode.dev/rss"));
    assert_source_definition(&source, "Hashnode");
    assert_lookup(&source, "testuser");
}

#[test]
fn tumblr_constructs_correct_feed_url() {
    let source = Tumblr::new("testuser").unwrap();
    assert!(source.feed_url().as_str().contains("testuser.tumblr.com/rss"));
    assert_source_definition(&source, "Tumblr");
    assert_lookup(&source, "testuser");
}

#[test]
fn letterboxd_constructs_correct_feed_url() {
    let source = Letterboxd::new("testuser").unwrap();
    assert!(source.feed_url().as_str().contains("letterboxd.com/testuser/rss/"));
    assert_source_definition(&source, "Letterboxd");
    assert_lookup(&source, "testuser");
}

#[test]
fn reddit_constructs_correct_feed_url() {
    let source = Reddit::new("testuser").unwrap();
    assert!(source.feed_url().as_str().contains("reddit.com/user/testuser/.rss"));
    assert_source_definition(&source, "Reddit");
    assert_lookup(&source, "testuser");
}

#[test]
fn deviant_art_constructs_correct_feed_url() {
    let source = DeviantArt::new("testuser").unwrap();
    assert!(source.feed_url().as_str().contains("backend.deviantart.com/rss.xml"));
    assert_source_definition(&source, "DeviantArt");
    assert_lookup(&source, "testuser");
}

#[test]
fn mixcloud_constructs_correct_feed_url() {
    let source = Mixcloud::new("testuser").unwrap();
    assert!(source.feed_url().as_str().contains("mixcloud.com/testuser/feed/"));
    assert_source_definition(&source, "Mixcloud");
    assert_lookup(&source, "testuser");
}

#[test]
fn vimeo_constructs_correct_feed_url() {
    let source = Vimeo::new("testuser").unwrap();
    assert!(source.feed_url().as_str().contains("vimeo.com/testuser/videos/rss"));
    assert_source_definition(&source, "Vimeo");
    assert_lookup(&source, "testuser");
}

#[test]
fn archive_of_our_own_constructs_correct_feed_url() {
    let source = ArchiveOfOurOwn::new("testuser").unwrap();
    assert!(source.feed_url().as_str().contains("archiveofourown.org/users/testuser/feed"));
    assert_source_definition(&source, "ArchiveOfOurOwn");
    assert_lookup(&source, "testuser");
}

#[test]
fn bit_chute_constructs_correct_feed_url() {
    let source = BitChute::new("testuser").unwrap();
    assert!(source.feed_url().as_str().contains("bitchute.com/feed/testuser"));
    assert_source_definition(&source, "BitChute");
    assert_lookup(&source, "testuser");
}

#[test]
fn odysee_constructs_correct_feed_url() {
    let source = Odysee::new("testuser").unwrap();
    assert!(source.feed_url().as_str().contains("odysee.com/$/rss/@testuser"));
    assert_source_definition(&source, "Odysee");
    assert_lookup(&source, "testuser");
}

#[test]
fn fetch_with_wrong_user_returns_none() {
    let source = Medium::new("testuser").unwrap();
    assert!(source.fetch_latest_post_by_user("wronguser").is_none());
    assert!(source.fetch_last_posts_by_user("wronguser", 5).is_empty());
}
