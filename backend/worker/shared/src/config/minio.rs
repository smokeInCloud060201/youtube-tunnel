use aws_config::{BehaviorVersion, Region as S3Region};
use aws_sdk_s3::Client;
use std::env;
use aws_sdk_s3::config::{Builder, Credentials};
use tracing::info;

pub async fn init() -> anyhow::Result<Client> {
    let aws_region = env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".into());
    let minio_url = env::var("MINIO_ENDPOINT").expect("ENDPOINT must be set");
    let minio_username = env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID must be set");
    let minio_password = env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY must be set");


    let base_config = aws_config::defaults(BehaviorVersion::latest())
        .region(S3Region::new(aws_region.clone()))
        .load()
        .await;


    let creds = Credentials::new(
        minio_username,
        minio_password,
        None,
        None,
        "static",
    );

    let config = Builder::from(&base_config)
        .region(S3Region::new(aws_region))
        .endpoint_url(&minio_url)
        .credentials_provider(creds)
        .force_path_style(true)
        .build();


    info!("Init MINIO with {}", minio_url);

    let client =  Client::from_conf(config);

    init_bucket(&client, String::from("videos")).await?;
    init_bucket(&client, String::from("credential")).await?;

    Ok(client)
}

async fn init_bucket(client: &Client, bucket_name: String) -> anyhow::Result<()> {
    if !bucket_exists(client, &bucket_name).await? {
        info!("Creating bucket with name {}", bucket_name);
        if let Err(e) = client.create_bucket().bucket(&bucket_name).send().await {
            tracing::error!("Failed to create bucket {}: {:?}", bucket_name, e);
            return Err(anyhow::anyhow!(e));
        }
        info!("Created bucket {} successfully", bucket_name);
    } else {
        info!("Bucket {} already exists", bucket_name);
    }

    Ok(())
}

async fn bucket_exists(client: &Client, bucket_name: &str) -> anyhow::Result<bool> {
    let resp = client.list_buckets().send().await?;

    let exists = resp
        .buckets()
        .iter()
        .any(|b| b.name().unwrap_or_default() == bucket_name);

    Ok(exists)
}
