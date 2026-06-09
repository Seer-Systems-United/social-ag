use reqwest::Url;

use crate::ParseType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Authentication {
    None,
    OptionalBearer,
    RequiredBearer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Capability {
    LookupUserById,
    LookupUserByUsername,
    LookupUserByDisplayName,
    FetchUserPosts,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceQuirk {
    AccountByIdRequiresStatusFallback,
    BoardAsUser,
    MastodonApiCompatible,
    UndocumentedPublicEndpoint,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceDefinition {
    pub name: &'static str,
    pub base_url: Url,
    pub protocol: ParseType,
    pub authentication: Authentication,
    pub capabilities: &'static [Capability],
    pub quirks: &'static [SourceQuirk],
}

impl SourceDefinition {
    pub fn supports(&self, capability: Capability) -> bool {
        self.capabilities.contains(&capability)
    }
}
