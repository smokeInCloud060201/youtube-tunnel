use std::env;

pub fn init() {
    unsafe {
        env::set_var(
            "RUST_LOG",
            "youtube_tunnel_api=info,youtube_tunnel_api=debug,actix_web=info,actix_web=debug,actix_server=info",
        );
        env::set_var(
            "RUST_BACKTRACE",
            "full",
        );
    }

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(false)
        .with_level(true)
        .with_ansi(true)
        .compact()
        .init();
}

