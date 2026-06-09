use crate::sources::mastodon;

mastodon::fixed_source!(
    TruthSocial,
    "https://truthsocial.com/",
    &[
        crate::SourceQuirk::MastodonApiCompatible,
        crate::SourceQuirk::AccountByIdRequiresStatusFallback,
    ]
);
