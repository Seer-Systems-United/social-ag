use social_ag::{Bandcamp, FourChan, Rutube, SocialSource, Telegram};

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
