use crate::sources::feed::feed_source;

feed_source!(
    MyAnimeList,
    "https://myanimelist.net/rss.php?type=rw&u={username}",
    "https://myanimelist.net/profile/{username}"
);
