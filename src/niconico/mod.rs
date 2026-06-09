use crate::sources::feed::feed_source;

feed_source!(
    Niconico,
    "https://www.nicovideo.jp/user/{username}/video?rss=2.0",
    "https://www.nicovideo.jp/user/{username}/video"
);
