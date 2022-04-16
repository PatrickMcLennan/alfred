
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client, Region};
use aws_sdk_s3::model::{Object};
use std::collections::HashMap;

pub struct S3 {}

impl S3 {
  pub async fn new() -> Client {
    let region_provider = RegionProviderChain::first_try(Region::new("us-east-1"))
        .or_default_provider()
        .or_else(Region::new("us-east-1"));
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    Client::new(&shared_config)
  }

  pub async fn get_items(bucket: String, client: Client) -> HashMap<String, Object> {
    let mut files = HashMap::new();

    client
      .list_objects_v2()
      .bucket(bucket)
      .send()
      .await
      .unwrap()
      .contents()
      .unwrap()
      .iter()
      .for_each(|f| {
        files.insert(f.key().unwrap().to_string(), f.clone());
        ()
      });

      files
  }

}