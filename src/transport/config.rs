use std::time::Duration;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(15);
const DEFAULT_CACHE_TTL: Duration = Duration::from_secs(30);
const DEFAULT_CIRCUIT_COOLDOWN: Duration = Duration::from_secs(30);
const DEFAULT_USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:128.0) Gecko/20100101 Firefox/128.0";

#[derive(Debug, Clone)]
pub struct TransportConfig {
    pub timeout: Duration,
    pub cache_ttl: Duration,
    pub max_retries: usize,
    pub max_retry_delay: Duration,
    pub max_concurrent_per_host: usize,
    pub circuit_failure_threshold: usize,
    pub circuit_cooldown: Duration,
    pub user_agent: String,
}

impl Default for TransportConfig {
    fn default() -> Self {
        Self {
            timeout: DEFAULT_TIMEOUT,
            cache_ttl: DEFAULT_CACHE_TTL,
            max_retries: 2,
            max_retry_delay: Duration::from_secs(5),
            max_concurrent_per_host: 4,
            circuit_failure_threshold: 3,
            circuit_cooldown: DEFAULT_CIRCUIT_COOLDOWN,
            user_agent: DEFAULT_USER_AGENT.to_string(),
        }
    }
}
