use actix_multipart::Multipart;
use actix_web::{post, web, HttpResponse};
use futures_util::StreamExt as _;
use crate::services::video::Video;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[post("/v1/video/clean")]
pub async fn clean_storage(service: web::Data<Arc<Video>>) -> actix_web::Result<HttpResponse> {
    let response = service
        .clean_storage()
        .await
        .expect("Failed to clean storage");

    Ok(HttpResponse::Ok().body(response))
}

#[post("/v1/video/cookie")]
pub async fn upload_cookie(
    mut payload: Multipart,
    service: web::Data<Arc<Video>>,
) -> actix_web::Result<HttpResponse> {
    let temp_path = std::env::temp_dir().join("uploaded-cookie.txt");
    let mut file = File::create(&temp_path)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    while let Some(Ok(mut field)) = payload.next().await {
        while let Some(Ok(chunk)) = field.next().await {
            file.write_all(&chunk)
                .await
                .map_err(actix_web::error::ErrorInternalServerError)?;
        }
    }

    file.flush()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    service
        .save_cookie(&temp_path)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    tokio::fs::remove_file(&temp_path).await.ok();

    Ok(HttpResponse::Ok().body("Cookie file uploaded successfully!"))
}

