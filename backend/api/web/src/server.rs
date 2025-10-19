use crate::controllers::video::{clean_storage, upload_cookie};
use crate::controllers::video_player::{get_video_playlist, request_video};
use crate::controllers::video_search::get_video;
use actix_cors::Cors;
use actix_web::middleware;
use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use listenfd::ListenFd;
use service::video::Video;
use service::video_job_producer::VideoJobProducer;
use service::video_player::VideoPlayer;
use service::video_search::VideoSearchService;
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

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(Arc::new(video_search.clone())))
            .app_data(web::Data::new(video.clone()))
            .app_data(web::Data::new(Arc::new(video_player.clone())))
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
        .service(get_video_playlist);
}
