use crate::sources::feed::feed_source;

feed_source!(
    Douban,
    "https://www.douban.com/feed/people/{username}/interests",
    "https://www.douban.com/people/{username}/"
);
