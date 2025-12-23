use deadpool_redis::{Config, Pool};
use std::env;
use tracing::log;

pub async fn init() -> anyhow::Result<Pool> {
    let redis_host = env::var("REDIS_HOST").expect("REDIS_HOST not set in .env file");
    let redis_port = env::var("REDIS_PORT").expect("REDIS_PORT not set in .env file");
    let redis_username = env::var("REDIS_USERNAME").unwrap_or_else(|_| "default".to_string());
    let redis_password = env::var("REDIS_PASSWORD").expect("REDIS_PASSWORD not set in .env file");
    
    // Format: redis://username:password@host:port/
    let redis_url = format!("redis://{}:{}@{}:{}/", redis_username, redis_password, redis_host, redis_port);
    let redis_pool_conf = Config::from_url(&redis_url);
    let redis_pool = redis_pool_conf
        .create_pool(None)
        .expect("Failed to create Redis pool");

    log::info!("Created Redis pool at redis://{}:***@{}:{}/", redis_username, redis_host, redis_port);

    Ok(redis_pool)
}

