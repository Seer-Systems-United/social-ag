pub mod mastodon;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParseType {
    Mastodon,
}
