use actix_multipart::Multipart;
use actix_web::{post, web, HttpResponse};
use futures_util::{StreamExt, TryStreamExt};
use crate::services::video::Video;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tracing::{error, info, warn};

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
    
    // Ensure we start with a fresh file (overwrite if exists)
    let mut file = File::create(&temp_path)
        .await
        .map_err(|e| {
            error!("Failed to create temp file: {}", e);
            actix_web::error::ErrorInternalServerError(e)
        })?;

    let mut bytes_written = 0u64;
    let mut field_count = 0u32;

    // Process multipart fields - consume the entire stream
    while let Some(field_result) = payload.try_next().await
        .map_err(|e| {
            error!("Error reading multipart stream: {}", e);
            actix_web::error::ErrorBadRequest(format!("Error parsing multipart data: {}", e))
        })? {
        field_count += 1;
        let field_name = field_result.name().map(|s| s.to_string()).unwrap_or_else(|| "unknown".to_string());
        let content_type = field_result.content_type().map(|ct| ct.to_string());
        
        info!(
            "Processing multipart field: name='{}', content_type='{:?}'",
            field_name, content_type
        );

        // Read all chunks from this field - consume the entire field stream
        let mut field_stream = field_result;
        while let Some(chunk) = field_stream.try_next().await
            .map_err(|e| {
                error!("Error reading chunk from field '{}': {}", field_name, e);
                actix_web::error::ErrorBadRequest(format!("Error reading file data: {}", e))
            })? {
            let chunk_size = chunk.len();
            file.write_all(&chunk)
                .await
                .map_err(|e| {
                    error!("Failed to write chunk to file: {}", e);
                    actix_web::error::ErrorInternalServerError(e)
                })?;
            bytes_written += chunk_size as u64;
        }
    }

    info!(
        "Finished reading multipart data: {} fields processed, {} bytes written",
        field_count, bytes_written
    );

    // Flush and explicitly close the file to ensure all data is written
    file.flush()
        .await
        .map_err(|e| {
            error!("Failed to flush file: {}", e);
            actix_web::error::ErrorInternalServerError(e)
        })?;
    drop(file); // Explicitly close the file

    // Verify file exists and has content before uploading
    let metadata = tokio::fs::metadata(&temp_path)
        .await
        .map_err(|e| {
            error!("Failed to get file metadata: {}", e);
            actix_web::error::ErrorInternalServerError(e)
        })?;
    
    info!("File size after write: {} bytes", metadata.len());
    
    if metadata.len() == 0 {
        warn!("No multipart fields received or file is empty");
        return Err(actix_web::error::ErrorBadRequest(
            format!(
                "Uploaded cookie file is empty. Received {} fields, wrote {} bytes",
                field_count, bytes_written
            )
        ));
    }

    // Save to MinIO (put_object will overwrite existing file with same key)
    service
        .save_cookie(&temp_path)
        .await
        .map_err(|e| {
            error!("Failed to save cookie to MinIO: {}", e);
            actix_web::error::ErrorInternalServerError(e)
        })?;

    tokio::fs::remove_file(&temp_path).await.ok();

    Ok(HttpResponse::Ok().body("Cookie file uploaded successfully!"))
}

