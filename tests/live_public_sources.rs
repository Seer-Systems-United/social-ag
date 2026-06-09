use social_ag::{Bandcamp, Farcaster, FourChan, Lens, Roblox, Rutube, SocialSource, Telegram};

#[test]
fn telegram_public_channels_work() {
    let source = Telegram::new("telegram").unwrap();
    let user = source
        .try_lookup_user_by_username("telegram")
        .unwrap()
        .unwrap();
    let posts = source.try_fetch_last_posts_by_user(&user.id, 2).unwrap();

    assert_eq!(posts.len(), 2);
    assert!(
        posts
            .iter()
            .all(|post| post.url.starts_with("https://t.me/"))
    );
}

#[test]
fn four_chan_public_catalogs_work() {
    let source = FourChan::new("g").unwrap();
    let user = source.try_lookup_user_by_username("g").unwrap().unwrap();
    let posts = source.try_fetch_last_posts_by_user(&user.id, 2).unwrap();

    assert_eq!(posts.len(), 2);
    assert!(
        posts
            .iter()
            .all(|post| post.url.starts_with("https://boards.4chan.org/g/"))
    );
}

#[test]
fn rutube_public_channels_work() {
    let source = Rutube::new("23178409").unwrap();
    let user = source.try_lookup_user_by_id("23178409").unwrap().unwrap();
    let posts = source.try_fetch_last_posts_by_user(&user.id, 2).unwrap();

    assert_eq!(posts.len(), 2);
    assert!(
        posts
            .iter()
            .all(|post| post.url.starts_with("https://rutube.ru/video/"))
    );
}

#[test]
fn bandcamp_public_catalogs_work() {
    let source = Bandcamp::new("sufjanstevens").unwrap();
    let post = source
        .try_fetch_latest_post_by_user("sufjanstevens")
        .unwrap()
        .unwrap();

    assert!(post.url.starts_with("https://"));
    assert!(!post.id.is_empty());
}

#[test]
fn roblox_public_games_work() {
    let source = Roblox::new("1").unwrap();
    let user = source.try_lookup_user_by_id("1").unwrap().unwrap();
    let post = source
        .try_fetch_latest_post_by_user(&user.id)
        .unwrap()
        .unwrap();

    assert_eq!(user.username, "Roblox");
    assert!(post.url.starts_with("https://www.roblox.com/games/"));
}

#[test]
fn farcaster_public_hub_works() {
    let source = Farcaster::new("2").unwrap();
    let user = source.try_lookup_user_by_id("2").unwrap().unwrap();
    let post = source
        .try_fetch_latest_post_by_user(&user.id)
        .unwrap()
        .unwrap();

    assert_eq!(user.username, "v");
    assert!(post.url.starts_with("https://farcaster.xyz/v/"));
}

#[test]
fn lens_public_graphql_works() {
    let source = Lens::new("lens").unwrap();
    let user = source.try_lookup_user_by_username("lens").unwrap().unwrap();
    let post = source
        .try_fetch_latest_post_by_user(&user.id)
        .unwrap()
        .unwrap();

    assert!(user.profile_url.starts_with("https://hey.xyz/u/"));
    assert!(post.url.starts_with("https://hey.xyz/posts/"));
}
