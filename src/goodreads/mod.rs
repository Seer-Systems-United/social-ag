use crate::sources::feed::feed_source;

feed_source!(
    Goodreads,
    "https://www.goodreads.com/user/updates_rss/{username}",
    "https://www.goodreads.com/user/show/{username}"
);
