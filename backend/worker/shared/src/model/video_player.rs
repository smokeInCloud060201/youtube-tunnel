use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VideoJob {
    #[serde(rename = "jobId")]
    pub job_id: String,

    #[serde(rename = "videoUrl")]
    pub video_url: String,
}
