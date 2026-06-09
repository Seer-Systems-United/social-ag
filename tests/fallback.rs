#[path = "support/source.rs"]
mod source;

use social_ag::FallbackChain;
use source::TestSource;

#[test]
fn falls_back_after_a_structured_block() {
    let chain = FallbackChain::new()
        .with_source(TestSource { blocked: true })
        .with_source(TestSource { blocked: false });
    let user = chain.try_lookup_user_by_username("alice").unwrap().unwrap();

    assert_eq!(user.username, "alice");
}
