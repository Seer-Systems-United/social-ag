use crate::sources::feed::feed_source;

feed_source!(
    StackOverflow,
    "https://stackoverflow.com/feeds/user/{username}",
    "https://stackoverflow.com/users/{username}"
);
