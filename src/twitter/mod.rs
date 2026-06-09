mod account;
mod config;
mod entities;
mod models;
mod posts;
mod source;
mod syndication;
mod syndication_models;

use reqwest::Url;

use crate::{Capability, transport::Transport};

const API_URL: &str = "https://api.x.com/";
const SYNDICATION_URL: &str = "https://syndication.twitter.com/srv/timeline-profile/screen-name/";
const CAPABILITIES: &[Capability] = &[
    Capability::LookupUserById,
    Capability::LookupUserByUsername,
    Capability::FetchUserPosts,
];

#[derive(Clone)]
pub struct Twitter {
    api_url: Url,
    syndication_url: Url,
    transport: Transport,
    authenticated: bool,
}
