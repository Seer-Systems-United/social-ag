pub(crate) fn transport_config() -> crate::TransportConfig {
    crate::TransportConfig {
        user_agent: concat!(
            "social-ag/",
            env!("CARGO_PKG_VERSION"),
            " (+",
            env!("CARGO_PKG_REPOSITORY"),
            ")"
        )
        .into(),
        ..Default::default()
    }
}
