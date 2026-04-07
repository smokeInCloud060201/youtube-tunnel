use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct QueryParams {
    #[serde(default = "default_part")]
    pub part: String,

    #[serde(default = "default_type")]
    #[serde(rename = "type")]
    pub resource_type: String,

    pub q: String,

    #[serde(default = "default_max_result")]
    pub max_result: u8,
}

fn default_part() -> String {
    "snippet".to_string()
}
fn default_type() -> String {
    "video".to_string()
}
fn default_max_result() -> u8 {
    50
}

#[derive(Serialize, Debug, Clone)]
pub struct VideoSearchResult {
    pub id: String,
    pub title: String,
    pub description: String,
    pub thumbnails: Vec<String>,
    #[serde(rename = "channelTitle")]
    pub channel_title: String,
    #[serde(rename = "publishTime")]
    pub publish_time: String,
    #[serde(rename = "publishedAt")]
    pub published_at: String,
}

impl VideoSearchResult {}

#[derive(Deserialize, Debug)]
pub struct SearchVideoResponseResult {
    // pub kind: String,
    pub etag: String,
    #[serde(rename = "nextPageToken")]
    pub next_page_token: String,
    #[serde(rename = "regionCode")]
    pub region_code: String,
    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
    pub items: Vec<Item>,
}

#[derive(Deserialize, Debug)]
pub struct PageInfo {
    #[serde(rename = "totalResults")]
    pub total_results: u32,
    #[serde(rename = "resultsPerPage")]
    pub results_per_page: u8,
}

#[derive(Deserialize, Debug)]
pub struct Item {
    pub kind: String,
    pub etag: String,
    pub id: Id,
    pub snippet: Snippet,
}

#[derive(Deserialize, Debug)]
pub struct Id {
    pub kind: String,
    #[serde(rename = "videoId")]
    pub video_id: String,
}

#[derive(Deserialize, Debug)]
pub struct Snippet {
    #[serde(rename = "publishedAt")]
    pub published_at: String,
    #[serde(rename = "channelId")]
    pub channel_id: String,
    pub title: String,
    pub description: String,
    pub thumbnails: Thumbnails,
    #[serde(rename = "channelTitle")]
    pub channel_title: String,
    #[serde(rename = "liveBroadcastContent")]
    pub live_broadcast_content: String,
    #[serde(rename = "publishTime")]
    pub publish_time: String,
}

#[derive(Deserialize, Debug)]
pub struct Thumbnails {
    #[serde(rename = "defaultThumbnail")]
    pub default_thumbnail: Option<Thumbnail>,
    pub medium: Option<Thumbnail>,
    pub high: Option<Thumbnail>,
}

#[derive(Deserialize, Debug)]
pub struct Thumbnail {
    pub url: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

