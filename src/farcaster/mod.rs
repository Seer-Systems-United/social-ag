mod account;
mod models;
mod posts;
mod source;

use reqwest::Url;

use crate::{
    SourceError, TransportConfig, parse::common::normalize_base_url, transport::Transport,
};

const HUB_URL: &str = "https://hub.pinata.cloud/v1/";

#[derive(Clone)]
pub struct Farcaster {
    hub_url: Url,
    fid: String,
    transport: Transport,
}

impl Farcaster {
    pub fn new(fid: impl AsRef<str>) -> Result<Self, SourceError> {
        Self::new_with_config(fid, TransportConfig::default())
    }

    pub fn new_with_config(
        fid: impl AsRef<str>,
        config: TransportConfig,
    ) -> Result<Self, SourceError> {
        let fid = fid.as_ref().trim().to_string();
        if fid.is_empty() || !fid.chars().all(|character| character.is_ascii_digit()) {
            return Err(SourceError::InvalidIdentifier(fid));
        }
        Ok(Self {
            hub_url: Url::parse(HUB_URL).unwrap(),
            fid,
            transport: Transport::new(config)?,
        })
    }

    pub fn with_hub_url(mut self, url: impl AsRef<str>) -> Result<Self, SourceError> {
        self.hub_url = normalize_base_url(url.as_ref())?;
        Ok(self)
    }

    pub fn fid(&self) -> &str {
        &self.fid
    }

    pub(super) fn endpoint(&self, path: &str) -> Url {
        let mut url = self.hub_url.clone();
        url.path_segments_mut().unwrap().pop_if_empty().push(path);
        url.query_pairs_mut().append_pair("fid", &self.fid);
        url
    }
}
