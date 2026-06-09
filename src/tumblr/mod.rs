use crate::sources::feed::feed_source;

feed_source!(
    Tumblr,
    "https://{username}.tumblr.com/rss",
    "https://{username}.tumblr.com"
);
