use serde::{Serialize, de::DeserializeOwned};

use crate::SourceError;

use super::{Lens, models::GraphResponse};

#[derive(Serialize)]
struct Request<'a, V> {
    query: &'a str,
    variables: V,
}

impl Lens {
    pub(super) fn query<T, V>(&self, query: &str, variables: V) -> Result<T, SourceError>
    where
        T: DeserializeOwned,
        V: Serialize,
    {
        let response: GraphResponse<T> = self
            .transport
            .post_json(self.api_url.clone(), &Request { query, variables })?;
        response.data.ok_or_else(|| {
            let message = response
                .errors
                .into_iter()
                .map(|error| error.message)
                .collect::<Vec<_>>()
                .join("; ");
            SourceError::InvalidResponse(message)
        })
    }
}
