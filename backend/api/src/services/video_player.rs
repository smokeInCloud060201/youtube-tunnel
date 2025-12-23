use crate::services::video_job_producer::VideoJobProducer;
use crate::model::video_player::JobStatusResponse;
use anyhow::{anyhow, bail};
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::Client;
use reqwest::Url;
use crate::model::video_player::VideoJob;
use std::time::Duration;
use tracing::{error, info};

#[derive(Clone)]
pub struct VideoPlayer {
    minio: Client,
    video_job_producer: VideoJobProducer,
}

impl VideoPlayer {
    pub fn new(minio: Client, video_job_producer: VideoJobProducer) -> Self {
        Self {
            minio,
            video_job_producer,
        }
    }

    pub async fn submit_video_job(&self, source_url: String) -> anyhow::Result<String> {
        let job_id = get_job_id(&source_url)?;

        // Check if video already exists
        let bucket_name = "yt-videos";
        let playlist_object = format!("{job_id}/playlist.m3u8");
        
        let video_exists = self
            .minio
            .head_object()
            .bucket(bucket_name)
            .key(&playlist_object)
            .send()
            .await
            .is_ok();

        if video_exists {
            info!("Video {} already exists, skipping queue", job_id);
            return Ok(job_id);
        }

        // Check if already processing
        if let Some(status) = self.video_job_producer.get_job_status(&job_id).await? {
            info!("Video {} status: {:?}, skipping queue", job_id, status);
            return Ok(job_id);
        }

        // Queue the job (will be deduplicated if already queued)
        let queued = self
            .video_job_producer
            .produce_job(
                VideoJob {
                    job_id: job_id.clone(),
                    video_url: source_url,
                },
                false,
            )
            .await?;

        if !queued {
            info!("Job {} was already queued or processing", job_id);
        }

        Ok(job_id)
    }

    pub async fn get_playlist(&self, job_id: String) -> anyhow::Result<String> {
        let bucket_name = "yt-videos";
        let playlist_object = format!("{job_id}/playlist.m3u8");

        self.minio
            .head_object()
            .bucket(bucket_name)
            .key(&playlist_object)
            .send()
            .await
            .map_err(|e| anyhow!("Playlist not found for job {job_id}: {e}"))?;

        let playlist_response = self
            .minio
            .get_object()
            .bucket(bucket_name)
            .key(&playlist_object)
            .send()
            .await?;

        let data = playlist_response.body.collect().await?;
        let playlist_content = String::from_utf8(data.into_bytes().to_vec())?;

        let mut updated_lines = Vec::new();

        for line in playlist_content.lines() {
            if line.ends_with(".ts") {
                let ts_object = format!("{job_id}/{line}");
                match self
                    .minio
                    .get_object()
                    .bucket(bucket_name)
                    .key(&ts_object)
                    .presigned(PresigningConfig::expires_in(Duration::from_secs(86400))?)
                    .await
                {
                    Ok(presigned_req) => {
                        let url = presigned_req.uri().to_string();
                        updated_lines.push(url);
                    }
                    Err(e) => {
                        error!("Failed to presign {}: {}", line, e);
                        updated_lines.push(line.to_string());
                    }
                }
            } else {
                updated_lines.push(line.to_string());
            }
        }

        let final_playlist = updated_lines.join("\n");
        info!("Generated presigned playlist for job_id={}", job_id);

        Ok(final_playlist)
    }

    pub async fn get_job_status(&self, job_id: &str) -> anyhow::Result<JobStatusResponse> {
        // Check if video exists first
        let bucket_name = "yt-videos";
        let playlist_object = format!("{job_id}/playlist.m3u8");
        
        let video_exists = self
            .minio
            .head_object()
            .bucket(bucket_name)
            .key(&playlist_object)
            .send()
            .await
            .is_ok();

        if video_exists {
            return Ok(JobStatusResponse {
                status: "completed".to_string(),
                progress: Some(1.0),
            });
        }

        // Get status from Redis
        let status = self.video_job_producer.get_job_status(job_id).await?;
        let progress = self.video_job_producer.get_job_progress(job_id).await?;

        let status_str = status
            .map(|s| s.as_str().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        Ok(JobStatusResponse {
            status: status_str,
            progress,
        })
    }
}

fn get_job_id(source_url: &String) -> anyhow::Result<String> {
    let url = Url::parse(&source_url)?;

    if let Some((_, value)) = url.query_pairs().find(|(k, _)| k == "v") {
        Ok(value.into_owned())
    } else {
        bail!("Video job URL missing")
    }
}

