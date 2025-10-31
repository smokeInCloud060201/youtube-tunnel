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
        let bucket_name = "videos";

        let list_response = self
            .minio
            .list_objects_v2()
            .bucket(bucket_name)
            .send()
            .await?;

        let contents: Vec<_> = list_response.contents().iter().cloned().collect();

        if contents.is_empty() {
            info!("Bucket '{}' is already empty.", bucket_name);
            return Ok("No objects to delete.".into());
        }

        let to_delete: Vec<ObjectIdentifier> = contents
            .iter()
            .filter_map(|obj| {
                obj.key()
                    .and_then(|k| ObjectIdentifier::builder().key(k).build().ok())
            })
            .collect();

        info!(
            "Deleting {} objects from bucket '{}'",
            to_delete.len(),
            bucket_name
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

        if failed.is_empty() {
            info!("Successfully deleted all objects from '{}'", bucket_name);
            Ok("Successfully deleted all objects".into())
        } else {
            error!("Failed to delete {} objects: {:?}", failed.len(), failed);
            Ok(format!(
                "Deleted some objects, but {} failed: {:?}",
                failed.len(),
                failed
            ))
        }
    }

    pub async fn save_cookie(&self, cookie_file_path: &Path) -> anyhow::Result<()> {
        let bucket_name = "credential";

        let contents = fs::read(cookie_file_path).await?;

        self.minio
            .put_object()
            .bucket(bucket_name)
            .key("cookie.txt")
            .body(contents.into())
            .send()
            .await?;

        info!("Uploaded cookie.txt to MinIO bucket `{}`", bucket_name);
        Ok(())
    }

    pub async fn cron_clean_storage(self: Arc<Self>) -> anyhow::Result<()> {
        let sched = JobScheduler::new().await?;

        let video_clone = Arc::clone(&self);
        sched
            .add(Job::new_async("0 0 23 * * *", move |_uuid, _l| {
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

