use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct QueryParams {
    #[serde(rename = "youtubeUrl")]
    pub youtube_url: String,
}

#[derive(Serialize, Debug)]
pub struct VideoPlayerResponse {
    #[serde(rename = "jobId")]
    pub job_id: String,
    #[serde(rename = "status")]
    pub status: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VideoJob {
    #[serde(rename = "jobId")]
    pub job_id: String,

    #[serde(rename = "videoUrl")]
    pub video_url: String,
}

#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum JobStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

impl JobStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            JobStatus::Pending => "pending",
            JobStatus::Processing => "processing",
            JobStatus::Completed => "completed",
            JobStatus::Failed => "failed",
        }
    }
}

#[derive(Serialize, Debug)]
pub struct JobStatusResponse {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "progress")]
    pub progress: Option<f64>,
}

