use crate::sources::feed::feed_source;

feed_source!(
    Dailymotion,
    "https://www.dailymotion.com/rss/user/{username}",
    "https://www.dailymotion.com/{username}"
);
