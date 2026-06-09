use std::time::Instant;

use super::Transport;

impl Transport {
    pub(super) fn fresh_cached_body(&self, key: &str) -> Option<Vec<u8>> {
        self.state
            .0
            .lock()
            .unwrap()
            .cache
            .get(key)
            .filter(|cached| cached.expires_at > Instant::now())
            .map(|cached| cached.body.clone())
    }

    pub(super) fn cached_body(&self, key: &str) -> Option<Vec<u8>> {
        self.state
            .0
            .lock()
            .unwrap()
            .cache
            .get(key)
            .map(|cached| cached.body.clone())
    }
}
