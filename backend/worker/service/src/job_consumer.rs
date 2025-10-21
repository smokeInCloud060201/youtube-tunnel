use anyhow::anyhow;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use deadpool_redis::redis::AsyncCommands;
use deadpool_redis::Pool;
use shared::config::cookie_credential::YoutubeCookie;
use shared::model::video_player::VideoJob;
use std::collections::HashSet;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::fs;
use tokio::process::Command;
use tokio::sync::Mutex;
use tokio::time::sleep;
use tracing::{error, info};

pub struct JobConsumer {
    minio: Client,
    redis_pool: Pool,
    youtube_cookie: YoutubeCookie,
    running: Arc<AtomicBool>,
}

impl JobConsumer {
    pub fn new(minio: Client, redis_pool: Pool, youtube_cookie: YoutubeCookie) -> Self {
        Self {
            minio,
            redis_pool,
            youtube_cookie,
            running: Arc::new(AtomicBool::new(true)),
        }
    }

    pub async fn start(self: Arc<Self>) -> anyhow::Result<()> {
        for i in 0..4 {
            let worker = self.clone();
            tokio::spawn(async move {
                if let Err(e) = worker.run_loop().await {
                    error!("Worker {i} crashed: {e:?}");
                }
            });
        }

        info!("VideoJobConsumer workers started!");
        Ok(())
    }

    pub async fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
        info!(" Stopping VideoJobConsumer workers...");
    }

    async fn run_loop(&self) -> anyhow::Result<()> {
        let mut conn = self.redis_pool.get().await?;
        while self.running.load(Ordering::SeqCst) {
            let result: Option<(String, String)> = conn
                .brpop("job-queue", 0f64)
                .await
                .map_err(|e| anyhow!("Redis pop failed: {e}"))?;

            if let Some((_queue, payload)) = result {
                match serde_json::from_str::<VideoJob>(&payload) {
                    Ok(job) => {
                        info!("Received job: {:?}", job);
                        if let Err(e) = self.handle_job(job).await {
                            error!("Failed to handle job: {e}");
                        }
                    }
                    Err(e) => error!("Failed to deserialize job payload: {e}"),
                }
            }
        }
        Ok(())
    }

    async fn handle_job(&self, job: VideoJob) -> anyhow::Result<()> {
        if job.job_id.is_empty()
            || self
                .object_exists("videos", &format!("{}/playlist.m3u8", job.job_id))
                .await
        {
            info!("Job is invalid or already exists: {}", job.job_id);
            return Ok(());
        }

        info!("Starting job {}", job.job_id);
        self.consume_data(&job).await?;
        Ok(())
    }

    async fn object_exists(&self, bucket: &str, key: &str) -> bool {
        self.minio
            .head_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await
            .is_ok()
    }

    async fn consume_data(&self, job: &VideoJob) -> anyhow::Result<()> {
        let bucket_name = "videos";
        let job_id = &job.job_id;
        let youtube_url = &job.video_url;

        let work_dir = std::env::temp_dir().join(format!("video-{}", job_id));
        fs::create_dir_all(&work_dir).await?;

        let playlist_path = work_dir.join("playlist.m3u8");

        let cookie_data = self.youtube_cookie.get_cookie_data().await?;
        let cookie_path = work_dir.join("cookie.txt");
        fs::write(&cookie_path, &cookie_data).await?;

        info!("Got cookie: {}", &cookie_path.to_string_lossy());

        // yt-dlp -> ffmpeg pipeline
        let mut yt_process = Command::new("yt-dlp")
            .args([
                "--no-playlist",
                "--cookies",
                &cookie_path.to_string_lossy(),
                "-f",
                "bv*[vcodec^=avc1]+ba/b",
                "-o",
                "-",
                youtube_url,
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;

        let mut ff_process = Command::new("ffmpeg")
            .args([
                "-i",
                "pipe:0",
                "-c:v",
                "libx264",
                "-preset",
                "fast",
                "-crf",
                "23",
                "-g",
                "60",
                "-keyint_min",
                "60",
                "-sc_threshold",
                "0",
                "-c:a",
                "aac",
                "-b:a",
                "128k",
                "-ac",
                "2",
                "-ar",
                "44100",
                "-af",
                "aresample=async=1",
                "-hls_time",
                "6",
                "-hls_list_size",
                "0",
                "-hls_flags",
                "independent_segments+append_list",
                "-start_number",
                "0",
                "-f",
                "hls",
                playlist_path.to_str().unwrap(),
            ])
            .stdin(Stdio::piped())
            .stderr(Stdio::inherit())
            .current_dir(&work_dir)
            .spawn()?;

        if let (Some(mut yt_out), Some(mut ff_in)) =
            (yt_process.stdout.take(), ff_process.stdin.take())
        {
            tokio::spawn(async move {
                let _ = tokio::io::copy(&mut yt_out, &mut ff_in).await;
            });
        }

        let uploaded_files = Arc::new(Mutex::new(HashSet::new()));
        let minio_clone = self.minio.clone();
        let uploaded_clone = uploaded_files.clone();
        let uploader_job_id = job_id.clone();
        let work_dir_clone = work_dir.clone();

        tokio::spawn(async move {
            let mut last_playlist_upload: Option<SystemTime> = None;

            loop {
                match fs::read_dir(&work_dir_clone).await {
                    Ok(mut dir) => {
                        while let Ok(Some(entry)) = dir.next_entry().await {
                            let file = entry.path();
                            let filename = file.file_name().unwrap().to_string_lossy().to_string();

                            // Upload segments (.ts)
                            if filename.ends_with(".ts") {
                                let mut uploaded = uploaded_clone.lock().await;
                                if uploaded.contains(&filename) {
                                    continue;
                                }

                                if !is_file_stable(&file, 300).await {
                                    continue;
                                }

                                let mut attempts = 0;
                                let uploaded_ok = loop {
                                    attempts += 1;
                                    match upload_file(
                                        &minio_clone,
                                        bucket_name,
                                        &uploader_job_id,
                                        &file,
                                    )
                                    .await
                                    {
                                        Ok(_) => break true,
                                        Err(e) => {
                                            error!(
                                                "Failed to upload {filename} (try {attempts}): {e}"
                                            );
                                            if attempts >= 3 {
                                                break false;
                                            }
                                            sleep(Duration::from_secs(2)).await;
                                        }
                                    }
                                };

                                if uploaded_ok {
                                    uploaded.insert(filename.clone());
                                    info!("Uploaded segment {filename}");
                                    let _ = fs::remove_file(&file).await;
                                }
                            }
                            // Upload playlist (.m3u8)
                            else if filename.ends_with(".m3u8") {
                                if let Ok(metadata) = fs::metadata(&file).await {
                                    let modified = metadata.modified().unwrap_or(SystemTime::now());

                                    let playlist_text =
                                        fs::read_to_string(&file).await.unwrap_or_default();
                                    let playlist_segments: Vec<String> = playlist_text
                                        .lines()
                                        .filter(|l| l.ends_with(".ts"))
                                        .map(|l| l.to_string())
                                        .collect();

                                    let uploaded = uploaded_clone.lock().await;
                                    let all_uploaded =
                                        playlist_segments.iter().all(|seg| uploaded.contains(seg));

                                    if all_uploaded
                                        && last_playlist_upload.map_or(true, |prev| modified > prev)
                                    {
                                        last_playlist_upload = Some(modified);
                                        if let Err(e) = upload_file(
                                            &minio_clone,
                                            bucket_name,
                                            &uploader_job_id,
                                            &file,
                                        )
                                        .await
                                        {
                                            error!("Failed to upload playlist: {e}");
                                        } else {
                                            info!("Uploaded updated playlist {filename}");
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("Upload scanner failed: {e}");
                        break;
                    }
                }

                sleep(Duration::from_secs(2)).await;
            }
        });

        let _ = yt_process.wait().await;
        let _ = ff_process.wait().await;

        if let Ok(mut dir) = fs::read_dir(&work_dir).await {
            while let Ok(Some(entry)) = dir.next_entry().await {
                let path = entry.path();
                if path.extension().map_or(false, |e| e == "ts" || e == "m3u8") {
                    if let Err(e) = upload_file(&self.minio, bucket_name, job_id, &path).await {
                        error!("Retrying upload of {:?}: {e}", path);
                        upload_file(&self.minio, bucket_name, job_id, &path).await?;
                    }
                }
            }
        }

        if fs::try_exists(&cookie_path).await? {
            fs::remove_file(&cookie_path).await?;
            info!("Deleted temp cookie file at {:?}", cookie_path);
        }

        if fs::try_exists(&playlist_path).await? {
            upload_file(&self.minio, bucket_name, job_id, &playlist_path).await?;
            info!(" Final playlist uploaded for job {}", job_id);
        }

        fs::remove_dir_all(&work_dir.clone()).await?;
        info!(" Job {} finished successfully!", job_id);
        Ok(())
    }
}

async fn upload_file(
    client: &Client,
    bucket: &str,
    job_id: &str,
    file: &PathBuf,
) -> anyhow::Result<()> {
    let key = format!("{}/{}", job_id, file.file_name().unwrap().to_string_lossy());
    let bytes = fs::read(file).await?;
    client
        .put_object()
        .bucket(bucket)
        .key(&key)
        .body(ByteStream::from(bytes))
        .send()
        .await?;
    Ok(())
}

async fn is_file_stable(path: &PathBuf, wait_ms: u64) -> bool {
    if let Ok(meta1) = fs::metadata(path).await {
        let size1 = meta1.len();
        tokio::time::sleep(Duration::from_millis(wait_ms)).await;
        if let Ok(meta2) = fs::metadata(path).await {
            return size1 == meta2.len();
        }
    }
    false
}