use anyhow::anyhow;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use deadpool_redis::redis::AsyncCommands;
use deadpool_redis::Pool;
use crate::config::cookie_credential::YoutubeCookie;
use crate::model::video_player::VideoJob;
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
            self.set_job_status(&job.job_id, "completed").await?;
            return Ok(());
        }

        info!("Starting job {}", job.job_id);
        self.set_job_status(&job.job_id, "processing").await?;
        self.set_job_progress(&job.job_id, 0.0).await?;

        match self.consume_data(&job).await {
            Ok(_) => {
                self.set_job_status(&job.job_id, "completed").await?;
                self.set_job_progress(&job.job_id, 1.0).await?;
                Ok(())
            }
            Err(e) => {
                error!("Job {} failed: {}", job.job_id, e);
                self.set_job_status(&job.job_id, "failed").await?;
                Err(e)
            }
        }
    }

    async fn set_job_status(&self, job_id: &str, status: &str) -> anyhow::Result<()> {
        let mut redis_conn = self.redis_pool.get().await?;
        let status_key = format!("job:{}:status", job_id);
        let _: () = redis_conn.set_ex::<_, _, ()>(&status_key, status, 86400).await?;
        Ok(())
    }

    async fn set_job_progress(&self, job_id: &str, progress: f64) -> anyhow::Result<()> {
        let mut redis_conn = self.redis_pool.get().await?;
        let progress_key = format!("job:{}:progress", job_id);
        let _: () = redis_conn
            .set_ex::<_, _, ()>(&progress_key, progress.to_string(), 86400)
            .await?;
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
        info!("Processing job {} (is_audio: {})", job_id, job.is_audio);

        // Convert cookie path to String to avoid temporary value issues
        let cookie_path_str = cookie_path.to_string_lossy().to_string();

        // yt-dlp -> ffmpeg pipeline
        let yt_dlp_args = if job.is_audio {
            // Audio-only download
            vec![
                "--no-playlist",
                "--cookies",
                &cookie_path_str,
                "-f",
                "ba/b",
                "-o",
                "-",
                youtube_url,
            ]
        } else {
            // Video + audio download
            vec![
                "--no-playlist",
                "--cookies",
                &cookie_path_str,
                "-f",
                "bv*[vcodec^=avc1]+ba/b",
                "-o",
                "-",
                youtube_url,
            ]
        };

        let mut yt_process = Command::new("yt-dlp")
            .args(yt_dlp_args)
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;

        let ffmpeg_args = if job.is_audio {
            // Audio-only HLS
            vec![
                "-i",
                "pipe:0",
                "-vn", // Disable video
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
            ]
        } else {
            // Video + audio HLS
            vec![
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
            ]
        };

        let mut ff_process = Command::new("ffmpeg")
            .args(ffmpeg_args)
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
        let redis_pool_clone = self.redis_pool.clone();

        tokio::spawn(async move {
            let mut last_playlist_upload: Option<SystemTime> = None;
            let mut segments_count = 0u64;

            loop {
                match fs::read_dir(&work_dir_clone).await {
                    Ok(mut dir) => {
                        let mut playlist_needs_upload = false;

                        while let Ok(Some(entry)) = dir.next_entry().await {
                            let file = entry.path();
                            let filename = file.file_name().unwrap().to_string_lossy().to_string();

                            // Upload segments (.ts)
                            if filename.ends_with(".ts") {
                                let mut uploaded = uploaded_clone.lock().await;
                                if uploaded.contains(&filename) {
                                    continue;
                                }

                                // For first few segments, use shorter wait time for faster playback start
                                let wait_time = if segments_count < 3 { 100 } else { 300 };
                                if !is_file_stable(&file, wait_time).await {
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
                                    segments_count += 1;
                                    info!("Uploaded segment {filename} (total: {})", segments_count);
                                    
                                    // Update progress (estimate: each segment ~0.05-0.1 of video, cap at 0.95 until completion)
                                    let progress = (segments_count as f64 * 0.05).min(0.95);
                                    let _ = set_job_progress_internal(&redis_pool_clone, &uploader_job_id, progress).await;
                                    
                                    playlist_needs_upload = true;
                                    let _ = fs::remove_file(&file).await;
                                }
                            }
                            // Upload playlist (.m3u8) - upload more frequently for better streaming
                            else if filename.ends_with(".m3u8") {
                                if let Ok(metadata) = fs::metadata(&file).await {
                                    let modified = metadata.modified().unwrap_or(SystemTime::now());
                                    
                                    // Upload playlist if:
                                    // 1. We have at least one segment uploaded, OR
                                    // 2. It's been 5 seconds since last upload, OR
                                    // 3. All segments are uploaded
                                    let should_upload = segments_count > 0 && (
                                        last_playlist_upload.map_or(true, |prev| {
                                            modified.duration_since(prev).unwrap_or(Duration::ZERO) > Duration::from_secs(5)
                                        }) || playlist_needs_upload
                                    );

                                    if should_upload {
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
                                            info!("Uploaded updated playlist {filename} ({} segments ready)", segments_count);
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

                sleep(Duration::from_secs(1)).await; // Check more frequently
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

async fn set_job_progress_internal(redis_pool: &Pool, job_id: &str, progress: f64) -> anyhow::Result<()> {
    let mut redis_conn = redis_pool.get().await?;
    let progress_key = format!("job:{}:progress", job_id);
    let _: () = redis_conn
        .set_ex::<_, _, ()>(&progress_key, progress.to_string(), 86400)
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

