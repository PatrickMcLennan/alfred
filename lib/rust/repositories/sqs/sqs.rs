
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_sqs::{Client, Region};
// use std::collections::HashMap;

pub struct SQS {}

impl SQS {
  pub async fn new() -> Client {
    let region_provider = RegionProviderChain::first_try(Region::new("us-east-1"))
      .or_default_provider()
      .or_else(Region::new("us-east-1"));
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    Client::new(&shared_config)
  }

  pub async fn get_queue_url(client: &Client, queue_name: String) -> String {
      client  
        .get_queue_url()
        .queue_name(queue_name)
        .send()
        .await
        .unwrap()
        .queue_url()
        .unwrap()
        .to_string()
  }
}