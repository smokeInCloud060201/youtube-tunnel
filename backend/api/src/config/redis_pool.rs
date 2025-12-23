use deadpool_redis::{Config, Pool};
use std::env;
use tracing::log;
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

fn url_encode(s: &str) -> String {
    percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string()
}

pub async fn init() -> anyhow::Result<Pool> {
    let redis_host = env::var("REDIS_HOST").expect("REDIS_HOST not set in .env file");
    let redis_port = env::var("REDIS_PORT").expect("REDIS_PORT not set in .env file");
    let redis_username = env::var("REDIS_USERNAME").unwrap_or_else(|_| "default".to_string());
    let redis_password = env::var("REDIS_PASSWORD").expect("REDIS_PASSWORD not set in .env file");
    
    // URL-encode username and password to handle special characters
    let encoded_username = url_encode(&redis_username);
    let encoded_password = url_encode(&redis_password);
    
    // Build Redis URL with username and password
    // Format: redis://username:password@host:port/
    // Always include username for Redis 6+ ACL compatibility
    let redis_url = format!("redis://{}:{}@{}:{}/", encoded_username, encoded_password, redis_host, redis_port);
    
    let redis_pool_conf = Config::from_url(&redis_url);
    let redis_pool = redis_pool_conf
        .create_pool(None)
        .map_err(|e| anyhow::anyhow!("Failed to create Redis pool: {}. URL format: redis://{}:***@{}:{}/", e, redis_username, redis_host, redis_port))?;

    log::info!("Created Redis pool at redis://{}:***@{}:{}/", redis_username, redis_host, redis_port);

    Ok(redis_pool)
}

