use aws_config::meta::region::RegionProviderChain;
use aws_sdk_cognitoidentityprovider::{Client, Region};

pub struct Cognito {}

impl Cognito {
  pub async fn new() -> Client {
    let region_provider = RegionProviderChain::first_try(Region::new("us-east-1"))
    .or_default_provider()
    .or_else(Region::new("us-east-1"));
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    Client::new(&shared_config)
  }
}