mod configurable;
mod fixed;
mod methods;

pub(crate) const CAPABILITIES: &[crate::Capability] = &[
    crate::Capability::LookupUserById,
    crate::Capability::LookupUserByUsername,
    crate::Capability::LookupUserByDisplayName,
    crate::Capability::FetchUserPosts,
];

pub(crate) use configurable::configurable_source;
pub(crate) use fixed::fixed_source;
pub(crate) use methods::source_methods;
