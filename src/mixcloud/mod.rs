use crate::sources::feed::feed_source;

feed_source!(
    Mixcloud,
    "https://www.mixcloud.com/{username}/feed/",
    "https://www.mixcloud.com/{username}/"
);
