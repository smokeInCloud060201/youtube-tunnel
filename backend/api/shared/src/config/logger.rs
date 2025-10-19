use std::env;

pub fn init() {
    unsafe {
        env::set_var(
            "RUST_LOG",
            "service=info,service=debug,shared=info,shared=debug,web=info,web=debug,actix_web=info,actix_web=debug,actix_server=info",
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
