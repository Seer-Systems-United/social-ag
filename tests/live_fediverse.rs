use social_ag::{BookWyrm, MicroBlog, Misskey, SocialSource};

fn assert_public_source(
    source: &impl SocialSource,
    username: &str,
    profile_host: &str,
    post_host: &str,
) {
    let user = source
        .try_lookup_user_by_username(username)
        .unwrap_or_else(|error| panic!("public username lookup failed: {error}"))
        .expect("public username should exist");
    assert!(user.profile_url.starts_with(profile_host));

    let user_by_id = source
        .lookup_user_by_id(&user.id)
        .expect("public account ID lookup should succeed");
    assert_eq!(user_by_id, user);

    let posts = source.fetch_last_posts_by_user(&user.id, 2);
    assert_eq!(posts.len(), 2);
    assert!(
        posts
            .iter()
            .all(|post| { post.publisher_user.id == user.id && post.url.starts_with(post_host) })
    );
}

#[test]
fn micro_blog_returns_public_users_and_posts() {
    assert_public_source(
        &MicroBlog::default(),
        "news",
        "https://micro.blog/",
        "https://news.micro.blog/",
    );
}

#[test]
fn misskey_returns_public_users_and_posts() {
    assert_public_source(
        &Misskey::new("https://misskey.io/").unwrap(),
        "syuilo",
        "https://misskey.io/",
        "https://misskey.io/",
    );
}

#[test]
fn bookwyrm_returns_public_users_and_posts() {
    assert_public_source(
        &BookWyrm::new("https://bookwyrm.social/").unwrap(),
        "mouse",
        "https://bookwyrm.social/",
        "https://bookwyrm.social/",
    );
}
