use social_ag::{Mastodon, ParseType, SourceRegistry, TruthSocial};

#[test]
fn registers_and_filters_sources() {
    let mut registry = SourceRegistry::new();
    registry.register(Mastodon::default()).unwrap();
    registry.register(TruthSocial::new()).unwrap();

    assert_eq!(registry.len(), 2);
    assert!(registry.get("truthsocial").is_some());
    assert_eq!(registry.by_protocol(ParseType::Mastodon).count(), 2);
}
