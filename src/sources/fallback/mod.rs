mod lookup;
mod policy;
mod posts;

use std::sync::Arc;

use crate::SocialSource;

pub type SharedSource = Arc<dyn SocialSource>;

#[derive(Clone, Default)]
pub struct FallbackChain {
    pub(super) sources: Vec<SharedSource>,
}

impl FallbackChain {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_source<S>(mut self, source: S) -> Self
    where
        S: SocialSource + 'static,
    {
        self.push(source);
        self
    }

    pub fn push<S>(&mut self, source: S)
    where
        S: SocialSource + 'static,
    {
        self.sources.push(Arc::new(source));
    }

    pub fn push_shared(&mut self, source: SharedSource) {
        self.sources.push(source);
    }

    pub fn len(&self) -> usize {
        self.sources.len()
    }

    pub fn is_empty(&self) -> bool {
        self.sources.is_empty()
    }
}

impl std::fmt::Debug for FallbackChain {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_list()
            .entries(self.sources.iter().map(|source| source.definition()))
            .finish()
    }
}
