use std::{collections::HashMap, time::Instant};

use reqwest::header::HeaderValue;

use super::Transport;

#[derive(Default)]
pub(super) struct TransportState {
    pub(super) cache: HashMap<String, CachedResponse>,
    pub(super) hosts: HashMap<String, HostState>,
}

pub(super) struct CachedResponse {
    pub(super) body: Vec<u8>,
    pub(super) etag: Option<HeaderValue>,
    pub(super) last_modified: Option<HeaderValue>,
    pub(super) expires_at: Instant,
}

#[derive(Default)]
pub(super) struct HostState {
    pub(super) active_requests: usize,
    pub(super) consecutive_failures: usize,
    pub(super) circuit_opened_at: Option<Instant>,
}

pub(super) struct HostPermit<'a> {
    transport: &'a Transport,
    host: String,
}

impl Transport {
    pub(super) fn acquire_host(&self, host: &str) -> HostPermit<'_> {
        let (lock, available) = &*self.state;
        let mut state = lock.lock().unwrap();
        let limit = self.config.max_concurrent_per_host.max(1);
        while state.hosts.entry(host.into()).or_default().active_requests >= limit {
            state = available.wait(state).unwrap();
        }
        state.hosts.entry(host.into()).or_default().active_requests += 1;
        HostPermit {
            transport: self,
            host: host.into(),
        }
    }
}

impl Drop for HostPermit<'_> {
    fn drop(&mut self) {
        let (lock, available) = &*self.transport.state;
        let mut state = lock.lock().unwrap();
        if let Some(host) = state.hosts.get_mut(&self.host) {
            host.active_requests = host.active_requests.saturating_sub(1);
        }
        available.notify_one();
    }
}
