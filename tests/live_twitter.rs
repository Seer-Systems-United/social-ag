use social_ag::{SocialSource, Twitter};

#[test]
fn reads_x_without_credentials() {
    let source = Twitter::default();
    let user = source
        .try_lookup_user_by_username("elonmusk")
        .unwrap()
        .unwrap();
    let posts = source.try_fetch_last_posts_by_user(&user.id, 2).unwrap();

    assert_eq!(user.username, "elonmusk");
    assert_eq!(posts.len(), 2);
    assert!(
        posts
            .iter()
            .all(|post| post.url.starts_with("https://x.com/"))
    );

    dbg!(user);
    dbg!(posts);
}
