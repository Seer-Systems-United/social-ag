use atrium_api::types::string::AtIdentifier;

use crate::{SourceError, parse::common::required_identifier};

pub(super) fn parse_actor_identifier(value: &str) -> Result<AtIdentifier, SourceError> {
    let value = required_identifier(value)?.trim_start_matches('@');
    value
        .parse()
        .map_err(|_| SourceError::InvalidIdentifier(value.into()))
}
