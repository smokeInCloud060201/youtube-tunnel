use crate::handlers::video::{clean_storage, upload_cookie};
use crate::handlers::video_player::{get_video_playlist, request_video, get_video_status, clean_job, clean_all_jobs};
use crate::handlers::video_search::get_video;
use actix_cors::Cors;
use actix_web::middleware;
use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use listenfd::ListenFd;
use crate::services::video::Video;
use crate::services::video_job_producer::VideoJobProducer;
use crate::services::video_player::VideoPlayer;
use crate::services::video_search::VideoSearchService;
use crate::config::{logger, minio, redis_pool};
use std::env;
use std::sync::Arc;
use tracing::{info, warn};
use tokio::time::{sleep, Duration};

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

    let video_search = VideoSearchService::new();
    let video = Arc::new(Video::new(minio_client.clone()));
    let video_job_producer = VideoJobProducer::new(redis_pool);
    let video_player = VideoPlayer::new(minio_client.clone(), video_job_producer.clone());

    match video.clone().cron_clean_storage().await {
        Ok(..) => {
            info!("Register cronjob successfully")
        }
        Err(err) => {
            info!("Register cronjob failed by {}", err)
        }
    };

    let mut server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_method()
            .allow_any_origin()
            .allow_any_header();

        // Configure payload size limits (10MB should be enough for cookie files)
        let payload_config = web::PayloadConfig::default()
            .limit(10 * 1024 * 1024); // 10MB

        App::new()
            .app_data(payload_config)
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(Arc::new(video_search.clone())))
            .app_data(web::Data::new(video.clone()))
            .app_data(web::Data::new(Arc::new(video_player.clone())))
            .app_data(web::Data::new(Arc::new(video_job_producer.clone())))
            .configure(init_config)
    });

    let mut listen_fd = ListenFd::from_env();
    server = if let Some(listener) = listen_fd.take_tcp_listener(0)? {
        server.listen(listener)?
    } else {
        server.bind(&server_url)?
    };

    info!("Starting server at {server_url}");
    server.run().await
}

fn init_config(cfg: &mut web::ServiceConfig) {
    cfg.service(request_video)
        .service(get_video)
        .service(clean_storage)
        .service(upload_cookie)
        .service(get_video_playlist)
        .service(get_video_status)
        .service(clean_job)
        .service(clean_all_jobs);
}

