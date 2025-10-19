use actix_web::get;
use actix_web::post;
use actix_web::web;
use actix_web::HttpResponse;
use service::video_player::VideoPlayer;
use shared::model::video_player::{QueryParams, VideoPlayerResponse};
use std::sync::Arc;
use tracing::info;

#[post("/v1/video-player")]
async fn request_video(
    params: web::Query<QueryParams>,
    service: web::Data<Arc<VideoPlayer>>,
) -> actix_web::Result<HttpResponse> {
    let source = params.into_inner().youtube_url;

    info!("Processing video {}", &source);

    let result = service
        .as_ref()
        .submit_video_job(source)
        .await
        .expect("Failed to submit video job");

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(VideoPlayerResponse { job_id: result }))
}

#[get("/v1/video-player/{job_id}/playlist")]
async fn get_video_playlist(
    job_id: web::Path<String>,
    service: web::Data<Arc<VideoPlayer>>,
) -> actix_web::Result<HttpResponse> {
    info!("Got a /v1/video-playlist request {job_id}");

    let response = service
        .as_ref()
        .get_playlist(job_id.to_string())
        .await
        .expect("Failed to get playlist");

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(response))
}
