use crate::sources::feed::feed_source;

feed_source!(Odysee, "https://odysee.com/$/rss/@{username}", "https://odysee.com/@{username}");
