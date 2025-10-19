use deadpool_redis::{Config, Pool};
use std::env;
use tracing::log;

pub async fn init() -> anyhow::Result<Pool> {
    let redis_host = env::var("REDIS_HOST").expect("REDIS not set in .env file");
    let redis_port = env::var("REDIS_PORT").expect("REDIS not set in .env file");
    let redis_url = format!("redis://{}:{}/", redis_host, redis_port);
    let redis_pool_conf = Config::from_url(&redis_url);
    let redis_pool = redis_pool_conf
        .create_pool(None)
        .expect("Failed to create Redis pool");

    log::info!("Created Redis pool at {}", &redis_url);

    Ok(redis_pool)
}
