use base64::engine::general_purpose;
use base64::Engine;
use futures::future::join_all;
use futures::join;
use reqwest::Client;
use crate::model::video_search::{
    QueryParams, SearchVideoResponseResult, Thumbnail, Thumbnails, VideoSearchResult,
};

#[derive(Clone)]
pub struct VideoSearchService {
    client: Client,
    api_key: String,
    url: String,
}

impl VideoSearchService {
    pub fn new() -> Self {
        let api_key =
            std::env::var("YOUTUBE_API_KEY").expect("YOUTUBE_API_KEY must be set in .env");
        let youtube_host = std::env::var("YOUTUBE_HOST").expect("YOUTUBE_HOST must be set in .env");

        Self {
            client: Client::new(),
            api_key,
            url: youtube_host,
        }
    }

    async fn get_best_thumbnail(&self, thumbs: &Thumbnails) -> anyhow::Result<Option<String>> {
        async fn fetch_and_encode(client: &Client, url: &str) -> anyhow::Result<String> {
            let bytes = client.get(url).send().await?.bytes().await?;
            Ok(general_purpose::STANDARD.encode(&bytes))
        }

        let fut_default = thumbs
            .default_thumbnail
            .as_ref()
            .map(|t| fetch_and_encode(&self.client, &t.url));
        let fut_medium = thumbs
            .medium
            .as_ref()
            .map(|t| fetch_and_encode(&self.client, &t.url));
        let fut_high = thumbs
            .high
            .as_ref()
            .map(|t| fetch_and_encode(&self.client, &t.url));

        let (default, medium, high) = join!(
            async {
                match fut_default {
                    Some(f) => f.await.ok(),
                    None => None,
                }
            },
            async {
                match fut_medium {
                    Some(f) => f.await.ok(),
                    None => None,
                }
            },
            async {
                match fut_high {
                    Some(f) => f.await.ok(),
                    None => None,
                }
            },
        );

        Ok(default.or(medium).or(high))
    }

    pub async fn search(&self, query: QueryParams) -> anyhow::Result<Vec<VideoSearchResult>> {
        let url = format!(
            "{}/youtube/v3/search?part={}&q={}&type={}&maxResults={}&key={}",
            self.url, query.part, query.q, query.resource_type, query.max_result, self.api_key
        );

        let result = self.client.get(&url).send().await?.text().await?;

        let search_result: SearchVideoResponseResult = serde_json::from_str(&result)?;

        let futures = search_result.items.into_iter().map(|item| async move {
            let thumbs = Thumbnails {
                default_thumbnail: item.snippet.thumbnails.default_thumbnail.as_ref().map(|t| {
                    Thumbnail {
                        url: t.url.clone(),
                        width: None,
                        height: None,
                    }
                }),
                medium: item.snippet.thumbnails.medium.as_ref().map(|t| Thumbnail {
                    url: t.url.clone(),
                    width: None,
                    height: None,
                }),
                high: item.snippet.thumbnails.high.as_ref().map(|t| Thumbnail {
                    url: t.url.clone(),
                    width: None,
                    height: None,
                }),
            };
            let best_thumb = self.get_best_thumbnail(&thumbs).await.ok().flatten();

            VideoSearchResult {
                id: item.id.video_id,
                title: item.snippet.title,
                description: item.snippet.description,
                thumbnails: best_thumb.into_iter().collect(),
                channel_title: item.snippet.channel_title,
                publish_time: item.snippet.publish_time,
                published_at: item.snippet.published_at,
            }
        });

        let response: Vec<VideoSearchResult> = join_all(futures).await;

        Ok(response)
    }
}

