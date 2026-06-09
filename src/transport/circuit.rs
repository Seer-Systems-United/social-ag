use std::time::Instant;

use crate::SourceError;

use super::Transport;

impl Transport {
    pub(super) fn ensure_circuit_closed(&self, host: &str) -> Result<(), SourceError> {
        let mut state = self.state.0.lock().unwrap();
        let host_state = state.hosts.entry(host.into()).or_default();
        if let Some(opened_at) = host_state.circuit_opened_at {
            if opened_at.elapsed() < self.config.circuit_cooldown {
                return Err(SourceError::CircuitOpen { host: host.into() });
            }
            host_state.consecutive_failures = 0;
            host_state.circuit_opened_at = None;
        }
        Ok(())
    }

    pub(super) fn record_success(&self, host: &str) {
        let mut state = self.state.0.lock().unwrap();
        let host_state = state.hosts.entry(host.into()).or_default();
        host_state.consecutive_failures = 0;
        host_state.circuit_opened_at = None;
    }

    pub(super) fn record_failure(&self, host: &str) {
        let mut state = self.state.0.lock().unwrap();
        let host_state = state.hosts.entry(host.into()).or_default();
        host_state.consecutive_failures += 1;
        if host_state.consecutive_failures >= self.config.circuit_failure_threshold.max(1) {
            host_state.circuit_opened_at = Some(Instant::now());
        }
    }
}
