use actix_web::get;
use actix_web::post;
use actix_web::delete;
use actix_web::web;
use actix_web::HttpResponse;
use crate::services::video_player::VideoPlayer;
use crate::services::video_job_producer::VideoJobProducer;
use crate::model::video_player::{QueryParams, VideoPlayerResponse, JobStatusResponse};
use std::sync::Arc;
use tracing::info;

#[post("/v1/video-player")]
async fn request_video(
    params: web::Query<QueryParams>,
    service: web::Data<Arc<VideoPlayer>>,
) -> actix_web::Result<HttpResponse> {
    let source = params.into_inner().youtube_url;

    info!("Processing video {}", &source);

    let job_id = service
        .as_ref()
        .submit_video_job(source)
        .await
        .expect("Failed to submit video job");

    // Get status to return to client
    let status_response = service
        .as_ref()
        .get_job_status(&job_id)
        .await
        .unwrap_or_else(|_| JobStatusResponse {
            status: "unknown".to_string(),
            progress: None,
        });

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(VideoPlayerResponse {
            job_id,
            status: status_response.status,
        }))
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

#[get("/v1/video-player/{job_id}/status")]
async fn get_video_status(
    job_id: web::Path<String>,
    service: web::Data<Arc<VideoPlayer>>,
) -> actix_web::Result<HttpResponse> {
    info!("Got a /v1/video-player/status request {job_id}");

    let response = service
        .as_ref()
        .get_job_status(&job_id)
        .await
        .unwrap_or_else(|_| JobStatusResponse {
            status: "unknown".to_string(),
            progress: None,
        });

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(response))
}

#[delete("/v1/video-player/jobs/{job_id}")]
pub async fn clean_job(
    job_id: web::Path<String>,
    producer: web::Data<Arc<VideoJobProducer>>,
) -> actix_web::Result<HttpResponse> {
    info!("Cleaning job: {}", job_id);
    
    match producer.clean_job(&job_id).await {
        Ok(deleted_count) => {
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .json(serde_json::json!({
                    "message": format!("Job {} cleaned successfully", job_id),
                    "deleted_keys": deleted_count
                })))
        }
        Err(e) => {
            tracing::error!("Failed to clean job {}: {}", job_id, e);
            Ok(HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(serde_json::json!({
                    "error": format!("Failed to clean job: {}", e)
                })))
        }
    }
}

#[delete("/v1/video-player/jobs")]
pub async fn clean_all_jobs(
    producer: web::Data<Arc<VideoJobProducer>>,
) -> actix_web::Result<HttpResponse> {
    info!("Cleaning all jobs");
    
    match producer.clean_all_jobs().await {
        Ok(deleted_count) => {
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .json(serde_json::json!({
                    "message": "All jobs cleaned successfully",
                    "deleted_keys": deleted_count
                })))
        }
        Err(e) => {
            tracing::error!("Failed to clean all jobs: {}", e);
            Ok(HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(serde_json::json!({
                    "error": format!("Failed to clean all jobs: {}", e)
                })))
        }
    }
}

