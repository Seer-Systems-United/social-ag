use crate::sources::feed::feed_source;

feed_source!(Reddit, "https://www.reddit.com/user/{username}/.rss", "https://www.reddit.com/user/{username}/");
