use std::{collections::BTreeMap, sync::Arc};

use crate::{FallbackChain, ParseType, SharedSource, SocialSource, SourceDefinition, SourceError};

#[derive(Clone, Default)]
pub struct SourceRegistry {
    sources: BTreeMap<String, SharedSource>,
}

impl SourceRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register<S>(&mut self, source: S) -> Result<Option<SharedSource>, SourceError>
    where
        S: SocialSource + 'static,
    {
        let name = source.definition().name.to_string();
        self.register_as(name, source)
    }

    pub fn register_as<S>(
        &mut self,
        name: impl AsRef<str>,
        source: S,
    ) -> Result<Option<SharedSource>, SourceError>
    where
        S: SocialSource + 'static,
    {
        let key = source_key(name.as_ref())?;
        Ok(self.sources.insert(key, Arc::new(source)))
    }

    pub fn get(&self, name: &str) -> Option<&dyn SocialSource> {
        self.sources
            .get(&name.trim().to_ascii_lowercase())
            .map(AsRef::as_ref)
    }

    pub fn definitions(&self) -> impl Iterator<Item = SourceDefinition> + '_ {
        self.sources.values().map(|source| source.definition())
    }

    pub fn by_protocol(&self, protocol: ParseType) -> impl Iterator<Item = &dyn SocialSource> + '_ {
        self.sources
            .values()
            .filter(move |source| source.parse_type() == protocol)
            .map(AsRef::as_ref)
    }

    pub fn fallback_chain<'a>(&self, names: impl IntoIterator<Item = &'a str>) -> FallbackChain {
        let mut chain = FallbackChain::new();
        for name in names {
            if let Some(source) = self.sources.get(&name.trim().to_ascii_lowercase()) {
                chain.push_shared(source.clone());
            }
        }
        chain
    }

    pub fn len(&self) -> usize {
        self.sources.len()
    }

    pub fn is_empty(&self) -> bool {
        self.sources.is_empty()
    }
}

impl std::fmt::Debug for SourceRegistry {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_map()
            .entries(
                self.sources
                    .iter()
                    .map(|(name, source)| (name, source.definition())),
            )
            .finish()
    }
}

fn source_key(name: &str) -> Result<String, SourceError> {
    let name = name.trim();
    if name.is_empty() {
        return Err(SourceError::InvalidIdentifier(name.to_string()));
    }
    Ok(name.to_ascii_lowercase())
}
