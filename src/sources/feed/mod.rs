mod constructor;
mod discovered;
mod source;

macro_rules! feed_source {
    ($name:ident, $feed_url_template:expr, $profile_url_template:expr) => {
        $crate::sources::feed::feed_source!(
            $name,
            $feed_url_template,
            $profile_url_template,
            $crate::Authentication::None,
            &[
                $crate::Capability::LookupUserById,
                $crate::Capability::LookupUserByUsername,
                $crate::Capability::LookupUserByDisplayName,
                $crate::Capability::FetchUserPosts,
            ]
        );
    };
    ($name:ident, $feed_url_template:expr, $profile_url_template:expr, $auth:expr, $caps:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            parser: $crate::parse::feed::Parser,
        }

        $crate::sources::feed::feed_constructor!($name, $feed_url_template, $profile_url_template);
        $crate::sources::feed::feed_social_source!($name, $auth, $caps);
    };
}

pub(crate) use constructor::feed_constructor;
pub(crate) use discovered::profile_feed_source;
pub(crate) use feed_source;
pub(crate) use source::feed_social_source;
