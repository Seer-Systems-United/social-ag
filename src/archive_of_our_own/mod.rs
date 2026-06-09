use crate::sources::feed::feed_source;

feed_source!(
    ArchiveOfOurOwn,
    "https://archiveofourown.org/users/{username}/feed",
    "https://archiveofourown.org/users/{username}/"
);
