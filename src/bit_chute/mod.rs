use crate::sources::feed::feed_source;

feed_source!(
    BitChute,
    "https://www.bitchute.com/feed/{username}",
    "https://www.bitchute.com/{username}"
);
