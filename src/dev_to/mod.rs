use crate::sources::feed::feed_source;

feed_source!(
    DevTo,
    "https://dev.to/feed/{username}",
    "https://dev.to/{username}"
);
