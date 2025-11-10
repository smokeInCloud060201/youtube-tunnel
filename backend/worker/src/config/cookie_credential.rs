use anyhow::anyhow;
use aws_sdk_s3::Client;

#[derive(Debug, Clone)]
pub struct YoutubeCookie {
    pub minio: Client,
}

impl YoutubeCookie {
    pub fn new(minio: Client) -> Self {
        Self { minio }
    }

    pub async fn get_cookie_data(&self) -> anyhow::Result<Vec<u8>> {
        let bucket_name = "yt-credential";
        let cookie_key = "cookie.txt";

        let list_resp = self
            .minio
            .list_objects_v2()
            .bucket(bucket_name)
            .send()
            .await?;

        let found = list_resp
            .contents()
            .iter()
            .any(|obj| obj.key().unwrap_or_default() == cookie_key);

        if !found {
            return Err(anyhow!("cookie.txt not found in bucket '{}'", bucket_name));
        }

        let get_resp = self
            .minio
            .get_object()
            .bucket(bucket_name)
            .key(cookie_key)
            .send()
            .await?;

        let data = get_resp.body.collect().await?;
        Ok(data.into_bytes().to_vec())
    }
}

