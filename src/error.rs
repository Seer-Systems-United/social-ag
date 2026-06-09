use std::{fmt, time::Duration};

#[derive(Debug)]
pub enum SourceError {
    AuthenticationRequired,
    Blocked { status: reqwest::StatusCode },
    CircuitOpen { host: String },
    Http { status: reqwest::StatusCode },
    InvalidIdentifier(String),
    InvalidResponse(String),
    InvalidUrl(String),
    Json(serde_json::Error),
    NotFound,
    RateLimited { retry_after: Option<Duration> },
    Request(reqwest::Error),
    Unavailable { status: reqwest::StatusCode },
    Unsupported { capability: &'static str },
}

impl fmt::Display for SourceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AuthenticationRequired => {
                formatter.write_str("the source requires authentication")
            }
            Self::Blocked { status } => {
                write!(
                    formatter,
                    "the source blocked the request with HTTP {status}"
                )
            }
            Self::CircuitOpen { host } => {
                write!(formatter, "requests to {host} are temporarily disabled")
            }
            Self::Http { status } => {
                write!(formatter, "the source returned HTTP {status}")
            }
            Self::InvalidIdentifier(identifier) => {
                write!(formatter, "invalid user identifier: {identifier:?}")
            }
            Self::InvalidResponse(message) => {
                write!(formatter, "invalid source response: {message}")
            }
            Self::InvalidUrl(url) => write!(formatter, "invalid source URL: {url}"),
            Self::Json(error) => write!(formatter, "invalid JSON response: {error}"),
            Self::NotFound => formatter.write_str("the requested resource was not found"),
            Self::RateLimited { retry_after } => match retry_after {
                Some(delay) => write!(
                    formatter,
                    "the source rate limited the request; retry after {} seconds",
                    delay.as_secs()
                ),
                None => formatter.write_str("the source rate limited the request"),
            },
            Self::Request(error) => write!(formatter, "source request failed: {error}"),
            Self::Unavailable { status } => {
                write!(
                    formatter,
                    "the source is temporarily unavailable (HTTP {status})"
                )
            }
            Self::Unsupported { capability } => {
                write!(formatter, "the source does not support {capability}")
            }
        }
    }
}

impl std::error::Error for SourceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Json(error) => Some(error),
            Self::Request(error) => Some(error),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for SourceError {
    fn from(error: reqwest::Error) -> Self {
        Self::Request(error)
    }
}

impl From<serde_json::Error> for SourceError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}
