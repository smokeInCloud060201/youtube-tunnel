use aws_config::meta::region::RegionProviderChain;
use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::Client;
use std::env;
use tracing::info;

pub async fn init() -> anyhow::Result<Client> {
    let aws_region = env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".into());
    let minio_url = env::var("MINIO_ENDPOINT").expect("ENDPOINT must be set");

    let region = RegionProviderChain::first_try(Region::new(aws_region));

    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region)
        .endpoint_url(&minio_url)
        .load()
        .await;

    info!("Init MINIO with {}", minio_url);

    let client = Client::new(&config);

    init_bucket(&client, String::from("videos")).await?;
    init_bucket(&client, String::from("credential")).await?;

    Ok(client)
}

async fn init_bucket(client: &Client, bucket_name: String) -> anyhow::Result<()> {
    if !bucket_exists(client, &bucket_name).await? {
        info!("Creating bucket with name {}", bucket_name);

        client.create_bucket().bucket(&bucket_name).send().await?;
        info!("Created bucket {} successfully", bucket_name);
    } else {
        info!("Bucket {} already exists", bucket_name);
    }

    Ok(())
}

async fn bucket_exists(client: &Client, bucket_name: &String) -> anyhow::Result<bool> {
    let buckets = client.list_buckets().send().await?;
    Ok(buckets
        .buckets()
        .iter()
        .any(|item| item.name().unwrap_or_default() == bucket_name))
}
