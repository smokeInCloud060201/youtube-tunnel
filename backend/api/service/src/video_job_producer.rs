use deadpool_redis::redis::AsyncTypedCommands;
use deadpool_redis::Pool;
use shared::model::video_player::VideoJob;
use tracing::info;

#[derive(Clone)]
pub struct VideoJobProducer {
    pub redis_pool: Pool,
}

impl VideoJobProducer {
    pub fn new(redis_pool: Pool) -> Self {
        Self { redis_pool }
    }

    pub async fn produce_job(&self, job: VideoJob) -> anyhow::Result<()> {
        let mut redis_conn = self.redis_pool.get().await?;
        let job_payload = serde_json::to_string(&job)?;

        let response = redis_conn.lpush("job-queue", job_payload).await?;
        info!("Produced video job: {}", &response);

        Ok(())
    }
}
