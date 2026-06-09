use social_ag::{Mastodon, ParseType, SocialSource, TruthSocial};

fn assert_public_source(source: &impl SocialSource, username: &str, expected_host: &str) {
    let user = source
        .lookup_user_by_username(username)
        .expect("public username lookup should succeed");

    assert!(!user.id.is_empty());
    assert!(!user.username.is_empty());
    assert!(user.profile_url.starts_with(expected_host));

    let user_by_id = source
        .lookup_user_by_id(&user.id)
        .expect("public account ID lookup should succeed");

    assert_eq!(user_by_id.id, user.id);
    assert_eq!(user_by_id.username, user.username);
    assert_eq!(user_by_id.profile_url, user.profile_url);

    let posts = source.fetch_last_posts_by_user(&user.id, 2);
    assert_eq!(posts.len(), 2);

    for post in posts {
        assert!(!post.id.is_empty());
        assert_eq!(post.publisher_user.id, user.id);
        assert_eq!(post.publisher_user.username, user.username);
        assert!(post.url.starts_with(expected_host));
    }
}

#[test]
fn mastodon_returns_public_users_and_posts() {
    assert_public_source(&Mastodon::default(), "Mastodon", "https://mastodon.social/");
}

#[test]
fn truth_social_returns_public_users_and_posts() {
    let source = TruthSocial::new();

    assert_eq!(source.instance_url().as_str(), "https://truthsocial.com/");
    assert_eq!(source.parse_type(), ParseType::Mastodon);
    assert_public_source(&source, "realDonaldTrump", "https://truthsocial.com/");
}
