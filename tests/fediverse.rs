use social_ag::{
    Akkoma, BookWyrm, Friendica, Funkwhale, GoToSocial, MicroBlog, Misskey, Mobilizon, Owncast,
    ParseType, Pixelfed, Pleroma, SocialSource, WriteFreely,
};

fn assert_adapter(source: impl SocialSource, name: &str) {
    let definition = source.definition();
    assert_eq!(definition.name, name);
    assert_eq!(definition.protocol, ParseType::ActivityPub);
    assert_eq!(definition.base_url.as_str(), "https://example.social/");
}

macro_rules! assert_adapters {
    ($($source:ident),+ $(,)?) => {
        $(assert_adapter($source::new("https://example.social").unwrap(), stringify!($source));)+
    };
}

#[test]
fn exposes_activitypub_platform_adapters() {
    assert_adapters!(
        Pixelfed,
        Pleroma,
        Misskey,
        Friendica,
        Akkoma,
        GoToSocial,
        BookWyrm,
        Mobilizon,
        WriteFreely,
        Funkwhale,
        Owncast,
        MicroBlog,
    );
}

#[test]
fn micro_blog_has_a_canonical_default() {
    assert_eq!(
        MicroBlog::default().definition().base_url.as_str(),
        "https://micro.blog/"
    );
}
