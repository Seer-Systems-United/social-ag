use social_ag::{HackerNews, SocialSource};

#[test]
fn rejects_bad_base_urls() {
    assert!(
        HackerNews::new()
            .unwrap()
            .with_base_url("not-a-url")
            .is_err()
    );
}

#[test]
fn reports_network_errors() {
    let source = HackerNews::new()
        .unwrap()
        .with_base_url("http://127.0.0.1:1")
        .unwrap();

    assert!(
        source
            .try_lookup_user_by_username("pg")
            .unwrap_err()
            .to_string()
            .contains("error sending request")
    );
}
