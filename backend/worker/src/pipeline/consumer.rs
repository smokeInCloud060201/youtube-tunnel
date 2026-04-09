use anyhow::anyhow;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use deadpool_redis::redis::AsyncCommands;
use deadpool_redis::Pool;
use crate::youtube::cookie::YoutubeCookie;
use shared::models::video_player::VideoJob;
use std::collections::HashSet;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::fs;
use tokio::process::Command;
use tokio::sync::Mutex;
use notify::{Watcher, RecursiveMode, EventKind, event::ModifyKind};
use tokio::sync::mpsc;
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
        while self.running.load(Ordering::SeqCst) {
            let mut conn = match self.redis_pool.get().await {
                Ok(c) => c,
                Err(e) => {
                    tracing::error!("Worker failed to get redis connection: {}", e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    continue;
                }
            };
            
            let result: Option<(String, String)> = match conn.brpop("job-queue", 5.0).await {
                Ok(res) => res,
                Err(e) => {
                    tracing::error!("Redis pop error, retrying: {}", e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    continue;
                }
            };

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
                .object_exists("yt-videos", &format!("{}/playlist.m3u8", job.job_id))
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
        let _: () = redis_conn.set_ex::<_, _, ()>(&status_key, status, 3600).await?;
        Ok(())
    }

    async fn set_job_progress(&self, job_id: &str, progress: f64) -> anyhow::Result<()> {
        let mut redis_conn = self.redis_pool.get().await?;
        let progress_key = format!("job:{}:progress", job_id);
        let _: () = redis_conn
            .set_ex::<_, _, ()>(&progress_key, progress.to_string(), 3600)
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
        let bucket_name = "yt-videos";
        let job_id = &job.job_id;
        let youtube_url = &job.video_url;

        let work_dir = std::env::temp_dir().join(format!("video-{}", job_id));
        fs::create_dir_all(&work_dir).await?;

        let playlist_path = work_dir.join("playlist.m3u8");

        let cookie_data = self.youtube_cookie.get_cookie_data().await?;
        let cookie_path = work_dir.join("cookie.txt");
        fs::write(&cookie_path, &cookie_data).await?;

        info!("Got cookie: {}", &cookie_path.to_string_lossy());

        let (tx, mut rx) = mpsc::channel::<notify::Event>(100);
        let tx_clone = tx.clone();
        
        let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
            if let Ok(event) = res {
                let _ = tx_clone.blocking_send(event);
            }
        })?;

        watcher.watch(&work_dir, RecursiveMode::NonRecursive)?;

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
                "independent_segments+append_list+temp_file",
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
        let mut segments_count = 0u64;
        let mut yt_done = false;
        let mut ff_done = false;
        let mut last_playlist_upload = tokio::time::Instant::now();
        let bucket_name_str = bucket_name.to_string();

        loop {
            tokio::select! {
                _ = yt_process.wait(), if !yt_done => {
                    yt_done = true;
                }
                _ = ff_process.wait(), if !ff_done => {
                    ff_done = true;
                    // Stop watching directory events
                    break;
                }
                Some(event) = rx.recv() => {
                    match event.kind {
                        EventKind::Create(_) | EventKind::Modify(_) => {
                            for path in event.paths {
                                let filename = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                                
                                if filename.ends_with(".ts") {
                                    let mut uploaded = uploaded_files.lock().await;
                                    if uploaded.contains(&filename) {
                                        continue;
                                    }
                                    
                                    if let Ok(meta) = fs::metadata(&path).await {
                                        if meta.len() > 0 {
                                            uploaded.insert(filename.clone());
                                            segments_count += 1;
                                            
                                            let m_clone = self.minio.clone();
                                            let b_clone = bucket_name_str.clone();
                                            let j_clone = job_id.to_string();
                                            let p_clone = path.clone();
                                            let r_clone = self.redis_pool.clone();
                                            
                                            tokio::spawn(async move {
                                                if let Err(e) = upload_file(&m_clone, &b_clone, &j_clone, &p_clone).await {
                                                    error!("Failed to upload {}: {}", filename, e);
                                                } else {
                                                    info!("Uploaded segment {} (total: {})", filename, segments_count);
                                                    let progress = (segments_count as f64 * 0.05).min(0.95);
                                                    let _ = set_job_progress_internal(&r_clone, &j_clone, progress).await;
                                                    let _ = fs::remove_file(&p_clone).await;
                                                }
                                            });
                                        }
                                    }
                                } else if filename.ends_with(".m3u8") {
                                    if last_playlist_upload.elapsed() > Duration::from_secs(5) && segments_count > 0 {
                                        last_playlist_upload = tokio::time::Instant::now();
                                        
                                        let m_clone = self.minio.clone();
                                        let b_clone = bucket_name_str.clone();
                                        let j_clone = job_id.to_string();
                                        let p_clone = path.clone();
                                        
                                        tokio::spawn(async move {
                                            if let Err(e) = upload_file(&m_clone, &b_clone, &j_clone, &p_clone).await {
                                                error!("Failed to upload playlist: {}", e);
                                            } else {
                                                info!("Uploaded updated playlist {} ({} segments ready)", filename, segments_count);
                                            }
                                        });
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        drop(watcher);

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
            info!("Final playlist uploaded for job {}", job_id);
        }

        fs::remove_dir_all(&work_dir).await?;
        info!("Job {} finished successfully!", job_id);
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
        .set_ex::<_, _, ()>(&progress_key, progress.to_string(), 3600)
        .await?;
    Ok(())
}



