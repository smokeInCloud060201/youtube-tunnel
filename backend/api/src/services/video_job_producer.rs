use deadpool_redis::redis::AsyncCommands;
use deadpool_redis::Pool;
use crate::model::video_player::{JobStatus, VideoJob};
use tracing::{info, warn};

#[derive(Clone)]
pub struct VideoJobProducer {
    pub redis_pool: Pool,
}

impl VideoJobProducer {
    pub fn new(redis_pool: Pool) -> Self {
        Self { redis_pool }
    }

    /// Check if job is already in queue or being processed
    pub async fn is_job_queued_or_processing(&self, job_id: &str) -> anyhow::Result<bool> {
        let mut redis_conn = self.redis_pool.get().await?;
        
        // Check if job status exists (means it's being processed or completed)
        let status_key = format!("job:{}:status", job_id);
        let exists: bool = redis_conn.exists::<_, bool>(&status_key).await?;
        
        if exists {
            return Ok(true);
        }

        // Check if job is in queue (scan the queue)
        let queue_items: Vec<String> = redis_conn.lrange::<_, Vec<String>>("job-queue", 0, -1).await?;
        for item in queue_items {
            match serde_json::from_str::<VideoJob>(&item) {
                Ok(queued_job) if queued_job.job_id == job_id => {
                    return Ok(true);
                }
                _ => {}
            }
        }

        Ok(false)
    }

    /// Set job status in Redis
    pub async fn set_job_status(&self, job_id: &str, status: JobStatus) -> anyhow::Result<()> {
        let mut redis_conn = self.redis_pool.get().await?;
        let status_key = format!("job:{}:status", job_id);
        
        // Set status with 24 hour expiry
        let _: () = redis_conn
            .set_ex(&status_key, status.as_str(), 86400)
            .await?;
        
        Ok(())
    }

    /// Get job status from Redis
    pub async fn get_job_status(&self, job_id: &str) -> anyhow::Result<Option<JobStatus>> {
        let mut redis_conn = self.redis_pool.get().await?;
        let status_key = format!("job:{}:status", job_id);
        
        let status_str: Option<String> = redis_conn.get::<_, Option<String>>(&status_key).await?;
        
        match status_str.as_deref() {
            Some("pending") => Ok(Some(JobStatus::Pending)),
            Some("processing") => Ok(Some(JobStatus::Processing)),
            Some("completed") => Ok(Some(JobStatus::Completed)),
            Some("failed") => Ok(Some(JobStatus::Failed)),
            _ => Ok(None),
        }
    }

    /// Set job progress (0.0 to 1.0)
    pub async fn set_job_progress(&self, job_id: &str, progress: f64) -> anyhow::Result<()> {
        let mut redis_conn = self.redis_pool.get().await?;
        let progress_key = format!("job:{}:progress", job_id);
        
        let _: () = redis_conn
            .set_ex(&progress_key, progress.to_string(), 86400)
            .await?;
        
        Ok(())
    }

    /// Get job progress
    pub async fn get_job_progress(&self, job_id: &str) -> anyhow::Result<Option<f64>> {
        let mut redis_conn = self.redis_pool.get().await?;
        let progress_key = format!("job:{}:progress", job_id);
        
        let progress_str: Option<String> = redis_conn.get::<_, Option<String>>(&progress_key).await?;
        
        progress_str
            .as_deref()
            .and_then(|s| s.parse().ok())
            .map_or(Ok(None), |p| Ok(Some(p)))
    }

    pub async fn produce_job(&self, job: VideoJob, force: bool) -> anyhow::Result<bool> {
        // Check if job is already queued or being processed (unless forced)
        if !force {
            if self.is_job_queued_or_processing(&job.job_id).await? {
                warn!("Job {} already queued or processing, skipping", job.job_id);
                return Ok(false);
            }
        }

        let mut redis_conn = self.redis_pool.get().await?;
        let job_payload = serde_json::to_string(&job)?;

        // Set status to pending before queueing
        self.set_job_status(&job.job_id, JobStatus::Pending).await?;

        // Use Redis SET to prevent duplicates, then add to queue
        // This ensures idempotency
        let queue_key = format!("job:{}:queued", job.job_id);
        let was_new: bool = redis_conn.set_nx::<_, _, bool>(&queue_key, "1").await?;
        
        if !was_new && !force {
            warn!("Job {} duplicate prevented", job.job_id);
            return Ok(false);
        }

        // Add expiry to queue marker (1 hour)
        let _: () = redis_conn.expire(&queue_key, 3600).await?;

        let response = redis_conn.lpush::<_, _, i64>("job-queue", job_payload).await?;
        info!("Produced video job: {} (queue length: {})", job.job_id, response);

        Ok(true)
    }

    /// Clean a specific job's data from Redis
    pub async fn clean_job(&self, job_id: &str) -> anyhow::Result<usize> {
        let mut redis_conn = self.redis_pool.get().await?;
        let mut deleted = 0;

        // Delete job status
        let status_key = format!("job:{}:status", job_id);
        if redis_conn.del::<_, i32>(&status_key).await? > 0 {
            deleted += 1;
        }

        // Delete job progress
        let progress_key = format!("job:{}:progress", job_id);
        if redis_conn.del::<_, i32>(&progress_key).await? > 0 {
            deleted += 1;
        }

        // Delete queue marker
        let queue_key = format!("job:{}:queued", job_id);
        if redis_conn.del::<_, i32>(&queue_key).await? > 0 {
            deleted += 1;
        }

        // Remove from queue if present
        let queue_items: Vec<String> = redis_conn.lrange::<_, Vec<String>>("job-queue", 0, -1).await?;
        for item in queue_items.iter() {
            match serde_json::from_str::<VideoJob>(item) {
                Ok(queued_job) if queued_job.job_id == job_id => {
                    redis_conn.lrem::<_, _, i32>("job-queue", 1, item).await?;
                    deleted += 1;
                    break;
                }
                _ => {}
            }
        }

        info!("Cleaned job {}: {} keys deleted", job_id, deleted);
        Ok(deleted)
    }

    /// Clean all job-related keys from Redis
    pub async fn clean_all_jobs(&self) -> anyhow::Result<usize> {
        let mut redis_conn = self.redis_pool.get().await?;
        let mut deleted = 0;

        // Get all job:* keys
        let pattern = "job:*";
        let keys: Vec<String> = redis_conn.keys::<_, Vec<String>>(pattern).await?;
        
        if !keys.is_empty() {
            let count = redis_conn.del::<_, i32>(keys).await?;
            deleted += count as usize;
        }

        // Clear the job queue
        let queue_length: i32 = redis_conn.llen::<_, i32>("job-queue").await?;
        if queue_length > 0 {
            redis_conn.del::<_, i32>("job-queue").await?;
            deleted += queue_length as usize;
        }

        info!("Cleaned all jobs: {} keys deleted", deleted);
        Ok(deleted)
    }
}

