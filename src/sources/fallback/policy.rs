use crate::SourceError;

pub(super) fn allows_fallback(error: &SourceError) -> bool {
    matches!(
        error,
        SourceError::AuthenticationRequired
            | SourceError::Blocked { .. }
            | SourceError::CircuitOpen { .. }
            | SourceError::Http { .. }
            | SourceError::InvalidResponse(_)
            | SourceError::Json(_)
            | SourceError::NotFound
            | SourceError::RateLimited { .. }
            | SourceError::Request(_)
            | SourceError::Unavailable { .. }
            | SourceError::Unsupported { .. }
    )
}
