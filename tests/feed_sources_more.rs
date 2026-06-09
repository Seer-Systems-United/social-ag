#[path = "support/feed_source.rs"]
mod feed_support;

use feed_support::assert_source;
use social_ag::{Dailymotion, Douban, Goodreads, MyAnimeList, Niconico, Pinterest, StackOverflow};

#[test]
fn constructs_more_user_feed_sources() {
    assert_source(
        Pinterest::new("testuser").unwrap(),
        "Pinterest",
        "pinterest.com/testuser/feed.rss",
    );
    assert_source(
        Dailymotion::new("testuser").unwrap(),
        "Dailymotion",
        "dailymotion.com/rss/user/testuser",
    );
    assert_source(
        MyAnimeList::new("testuser").unwrap(),
        "MyAnimeList",
        "myanimelist.net/rss.php",
    );
    assert_source(
        Goodreads::new("testuser").unwrap(),
        "Goodreads",
        "goodreads.com/user/updates_rss/testuser",
    );
    assert_source(
        StackOverflow::new("testuser").unwrap(),
        "StackOverflow",
        "stackoverflow.com/feeds/user/testuser",
    );
    assert_source(
        Niconico::new("testuser").unwrap(),
        "Niconico",
        "nicovideo.jp/user/testuser/video",
    );
    assert_source(
        Douban::new("testuser").unwrap(),
        "Douban",
        "douban.com/feed/people/testuser/interests",
    );
}
