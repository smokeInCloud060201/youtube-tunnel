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
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VideoJob {
    #[serde(rename = "jobId")]
    pub job_id: String,

    #[serde(rename = "videoUrl")]
    pub video_url: String,
}
