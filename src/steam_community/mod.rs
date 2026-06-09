use crate::{Authentication, Capability, SourceQuirk, sources::feed::feed_source};

feed_source!(
    SteamCommunity,
    "https://steamcommunity.com/groups/{username}/rss",
    "https://steamcommunity.com/groups/{username}",
    Authentication::None,
    &[
        Capability::LookupUserById,
        Capability::LookupUserByUsername,
        Capability::FetchUserPosts,
    ],
    &[SourceQuirk::CommunityAsUser]
);
