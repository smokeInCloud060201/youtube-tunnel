use crate::video_job_producer::VideoJobProducer;
use anyhow::{anyhow, bail};
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::Client;
use reqwest::Url;
use shared::model::video_player::VideoJob;
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

        self.video_job_producer
            .produce_job(VideoJob {
                job_id: job_id.clone(),
                video_url: source_url,
            })
            .await?;

        Ok(job_id)
    }

    pub async fn get_playlist(&self, job_id: String) -> anyhow::Result<String> {
        let bucket_name = "videos";
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
}

fn get_job_id(source_url: &String) -> anyhow::Result<String> {
    let url = Url::parse(&source_url)?;

    if let Some((_, value)) = url.query_pairs().find(|(k, _)| k == "v") {
        Ok(value.into_owned())
    } else {
        bail!("Video job URL missing")
    }
}
