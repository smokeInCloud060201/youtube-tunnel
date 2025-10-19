use actix_web::middleware;
use actix_web::App;
use actix_web::HttpServer;
use listenfd::ListenFd;
use service::job_consumer::JobConsumer;
use shared::config::cookie_credential::YoutubeCookie;
use shared::config::{logger, minio, redis_pool};
use std::env;
use std::sync::Arc;
use tracing::info;

pub async fn start() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    logger::init();

    let server_host = env::var("SERVER_HOST").expect("HOST not set in .env file");
    let server_port = env::var("SERVER_PORT").expect("PORT not set in .env file");
    let server_url = format!("{server_host}:{server_port}");

    let redis_pool = redis_pool::init()
        .await
        .expect("Failed to create Redis pool");
    let minio_client = minio::init().await.expect("Failed to initialize minio");

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
