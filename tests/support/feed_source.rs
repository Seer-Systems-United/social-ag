use social_ag::{Capability, ParseType, SocialSource};

pub fn assert_source(source: impl SocialSource, name: &str, url_fragment: &str) {
    let definition = source.definition();
    assert_eq!(definition.name, name);
    assert_eq!(definition.protocol, ParseType::Feed);
    assert!(
        definition
            .capabilities
            .contains(&Capability::FetchUserPosts)
    );
    assert!(definition.base_url.as_str().contains(url_fragment));

    let user = source.lookup_user_by_username("testuser").unwrap();
    assert_eq!(user.id, "testuser");
    assert_eq!(source.lookup_user_by_id("testuser").unwrap(), user);
}
