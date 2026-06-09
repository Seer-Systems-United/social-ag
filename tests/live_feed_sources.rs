use social_ag::{
    Dailymotion, Douban, Flickr, Goodreads, MyAnimeList, Niconico, Pinterest, ProductHunt,
    SocialSource, SoundCloud, StackOverflow, SteamCommunity, YouTube,
};

fn assert_public_source(source: &impl SocialSource, identifier: &str, post_host: &str) {
    let user = source
        .try_lookup_user_by_username(identifier)
        .unwrap_or_else(|error| panic!("public lookup failed for {identifier}: {error}"))
        .expect("public user should exist");
    let post = source
        .try_fetch_latest_post_by_user(&user.id)
        .unwrap_or_else(|error| panic!("public feed failed for {identifier}: {error}"))
        .expect("public user should have a post");

    assert_eq!(post.publisher_user.id, user.id);
    assert!(post.url.starts_with(post_host));
}

#[test]
fn public_profile_feeds_work() {
    assert_public_source(
        &YouTube::new("GoogleDevelopers").unwrap(),
        "GoogleDevelopers",
        "https://www.youtube.com/",
    );
    assert_public_source(
        &Flickr::new("flickr").unwrap(),
        "flickr",
        "https://www.flickr.com/",
    );
    assert_public_source(
        &SoundCloud::new("monstercat").unwrap(),
        "monstercat",
        "https://soundcloud.com/",
    );
}

#[test]
fn public_fixed_feeds_work() {
    assert_public_source(
        &Pinterest::new("pinterest").unwrap(),
        "pinterest",
        "https://",
    );
    assert_public_source(
        &Dailymotion::new("dailymotion").unwrap(),
        "dailymotion",
        "https://www.dailymotion.com/",
    );
    assert_public_source(
        &MyAnimeList::new("spacecowboy").unwrap(),
        "spacecowboy",
        "https://myanimelist.net/",
    );
    assert_public_source(
        &Goodreads::new("1").unwrap(),
        "1",
        "https://www.goodreads.com/",
    );
    assert_public_source(
        &StackOverflow::new("22656").unwrap(),
        "22656",
        "https://stackoverflow.com/",
    );
    assert_public_source(
        &Niconico::new("1086055").unwrap(),
        "1086055",
        "https://www.nicovideo.jp/",
    );
    assert_public_source(&Douban::new("ahbei").unwrap(), "ahbei", "https://");
    assert_public_source(
        &SteamCommunity::new("GrabFreeGames").unwrap(),
        "GrabFreeGames",
        "https://steamcommunity.com/",
    );
    assert_public_source(
        &ProductHunt::new().unwrap(),
        "producthunt",
        "https://www.producthunt.com/",
    );
}
