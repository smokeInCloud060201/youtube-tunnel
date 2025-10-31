use actix_web::middleware;
use actix_web::App;
use actix_web::HttpServer;
use listenfd::ListenFd;
use crate::services::job_consumer::JobConsumer;
use crate::config::cookie_credential::YoutubeCookie;
use crate::config::{logger, minio, redis_pool};
use std::env;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::{info, warn};

pub async fn start() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    logger::init();

    let server_host = env::var("SERVER_HOST").expect("HOST not set in .env file");
    let server_port = env::var("SERVER_PORT").expect("PORT not set in .env file");
    let server_url = format!("{server_host}:{server_port}");

    let mut attempts = 0;
    let max_attempts = 10;

    let redis_pool = loop {
        match redis_pool::init().await {
            Ok(pool) => break pool,
            Err(err) => {
                attempts += 1;
                if attempts >= max_attempts {
                    panic!("Redis not ready after {} attempts: {}", max_attempts, err);
                }
                warn!("Redis not ready, retrying in 3s: {}", err);
                sleep(Duration::from_secs(3)).await;
            }
        }
    };

    attempts = 0;
    let minio_client = loop {
        match minio::init().await {
            Ok(client) => break client,
            Err(err) => {
                attempts += 1;
                if attempts >= max_attempts {
                    panic!("MinIO not ready after {} attempts: {}", max_attempts, err);
                }
                warn!("MinIO not ready, retrying in 3s: {}", err);
                sleep(Duration::from_secs(3)).await;
            }
        }
    };

    let cookie_credential = YoutubeCookie::new(minio_client.clone());

    let consumer = Arc::new(JobConsumer::new(
        minio_client.clone(),
        redis_pool.clone(),
        cookie_credential.clone(),
    ));
    let consumer_clone = consumer.clone();

    tokio::spawn(async move {
        consumer_clone.start().await.unwrap();
    });

    let mut server = HttpServer::new(move || App::new().wrap(middleware::Logger::default()));

    let mut listen_fd = ListenFd::from_env();
    server = if let Some(listener) = listen_fd.take_tcp_listener(0)? {
        server.listen(listener)?
    } else {
        server.bind(&server_url)?
    };

    info!("Starting server at {server_url}");
    server.run().await
}

