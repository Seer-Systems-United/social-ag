use crate::sources::feed::feed_source;

feed_source!(
    Pinterest,
    "https://www.pinterest.com/{username}/feed.rss",
    "https://www.pinterest.com/{username}/"
);
