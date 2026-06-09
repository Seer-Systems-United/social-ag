use crate::sources::feed::feed_source;

feed_source!(DeviantArt, "https://backend.deviantart.com/rss.xml?type=deviation&q=by%3A{username}", "https://www.deviantart.com/{username}");
