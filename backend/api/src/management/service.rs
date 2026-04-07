use anyhow::anyhow;
use aws_sdk_s3::types::ObjectIdentifier;
use aws_sdk_s3::Client;
use std::path::Path;
use std::sync::Arc;
use tokio::fs;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{error, info};

#[derive(Clone)]
pub struct Video {
    minio: Client,
}

impl Video {
    pub fn new(minio: Client) -> Self {
        Self { minio }
    }

    pub async fn clean_storage(&self) -> anyhow::Result<String> {
        let bucket_name = "yt-videos";
        let mut total_deleted = 0u64;
        let mut total_failed = 0u64;
        let mut all_failed: Vec<String> = Vec::new();
        let max_delete_batch = 1000u64;
        let mut continuation_token: Option<String> = None;

        loop {
            let mut list_builder = self
                .minio
                .list_objects_v2()
                .bucket(bucket_name)
                .max_keys(1000);

            if let Some(token) = &continuation_token {
                list_builder = list_builder.continuation_token(token);
            }

            let list_response = list_builder.send().await?;
            let contents: Vec<_> = list_response.contents().iter().cloned().collect();

            if contents.is_empty() {
                if total_deleted == 0 {
                    info!("Bucket '{}' is already empty.", bucket_name);
                    return Ok("No objects to delete.".into());
                } else {
                    info!(
                        "Successfully deleted all {} objects from '{}'",
                        total_deleted, bucket_name
                    );
                    if total_failed > 0 {
                        return Ok(format!(
                            "Deleted {} objects, but {} failed: {:?}",
                            total_deleted, total_failed, all_failed
                        ));
                    }
                    return Ok(format!("Successfully deleted all {} objects", total_deleted));
                }
            }

            for chunk in contents.chunks(max_delete_batch as usize) {
                let to_delete: Vec<ObjectIdentifier> = chunk
                    .iter()
                    .filter_map(|obj| {
                        obj.key()
                            .and_then(|k| ObjectIdentifier::builder().key(k).build().ok())
                    })
                    .collect();

                if to_delete.is_empty() {
                    continue;
                }

                info!(
                    "Deleting batch of {} objects from bucket '{}' (total deleted so far: {})",
                    to_delete.len(),
                    bucket_name,
                    total_deleted
                );

                let delete_response = self
                    .minio
                    .delete_objects()
                    .bucket(bucket_name)
                    .delete(
                        aws_sdk_s3::types::Delete::builder()
                            .set_objects(Some(to_delete))
                            .build()?,
                    )
                    .send()
                    .await?;

                let deleted = delete_response.deleted();
                total_deleted += deleted.len() as u64;

                let failed: Vec<String> = delete_response
                    .errors()
                    .iter()
                    .map(|err| {
                        format!(
                            "{} -> {}",
                            err.key().unwrap_or_default(),
                            err.message().unwrap_or_default()
                        )
                    })
                    .collect();

                if !failed.is_empty() {
                    let failed_count = failed.len();
                    total_failed += failed_count as u64;
                    all_failed.extend(failed);
                    error!(
                        "Failed to delete {} objects in this batch",
                        failed_count
                    );
                }
            }

            if list_response.is_truncated().unwrap_or(false) {
                continuation_token = list_response.next_continuation_token().map(|s| s.to_string());
            } else {
                break;
            }
        }

        if total_failed > 0 {
            Ok(format!(
                "Deleted {} objects, but {} failed: {:?}",
                total_deleted, total_failed, all_failed
            ))
        } else {
            Ok(format!("Successfully deleted all {} objects", total_deleted))
        }
    }

    pub async fn save_cookie(&self, cookie_file_path: &Path) -> anyhow::Result<()> {
        let bucket_name = "yt-credential";
        let cookie_key = "cookie.txt";

        let contents = fs::read(cookie_file_path).await?;
        
        if contents.is_empty() {
            return Err(anyhow!("Cookie file is empty"));
        }

        info!(
            "Uploading cookie file to MinIO: bucket={}, key={}, size={} bytes",
            bucket_name,
            cookie_key,
            contents.len()
        );

        // put_object will overwrite existing object with the same key
        self.minio
            .put_object()
            .bucket(bucket_name)
            .key(cookie_key)
            .body(contents.into())
            .send()
            .await?;

        info!(
            "Successfully uploaded cookie.txt to MinIO bucket `{}` (overwrote existing file if present)",
            bucket_name
        );
        Ok(())
    }

    pub async fn cron_clean_storage(self: Arc<Self>) -> anyhow::Result<()> {
        let sched = JobScheduler::new().await?;

        let video_clone = Arc::clone(&self);
        // Cron format: second minute hour day-of-month month day-of-week
        // "0 0 23 * * 0" = Every Sunday at 11:00:00 PM
        sched
            .add(Job::new_async("0 0 23 * * 0", move |_uuid, _l| {
                let video_clone = Arc::clone(&video_clone);
                Box::pin(async move {
                    match video_clone.clean_storage().await {
                        Ok(msg) => info!("Cronjob executed: {}", msg),
                        Err(err) => error!("Cronjob failed: {:?}", err),
                    }
                })
            })?)
            .await?;

        tokio::spawn(async move {
            if let Err(e) = sched.start().await {
                error!("Scheduler failed: {:?}", e);
            }
        });

        Ok(())
    }
}

