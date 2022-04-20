#[macro_use]
extern crate dotenv_codegen;

use aws_sdk_s3::types::ByteStream;
use lambda_runtime::{LambdaEvent, Error};
use serde::Deserialize;
use lib::{repositories::*, models::BlurhashQueueOutputItem};
use lib::services::*;
use std::time::SystemTime;
use aws_sdk_dynamodb::model::{AttributeValue};

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Body {
  pub receiptHandle: String,
  pub body: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct SqsEvent {
  pub Records: Option<Vec<Body>>
}

async fn handler(event: LambdaEvent<SqsEvent>) -> Result<(), Error> {
  let table_name = dotenv!("COLLECTOR_DYNAMODB").to_string();
  let bucket_name = dotenv!("WIDESCREEN_WALLPAPERS_BUCKET_NAME").to_string();
  let download_wallpaper_queue_name = dotenv!("COLLECTOR_DOWNLOAD_WALLPAPER_QUEUE_NAME").to_string();

  let time_stamp = SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .unwrap()
    .as_secs();

  let sqs_client = SQS::new().await;
  let s3_client = S3::new().await;
  let dynamo_client = DynamoDB::new().await;

  let queue_url = SQS::get_queue_url(&sqs_client, download_wallpaper_queue_name).await;

  let messages = event.payload.Records;
  let res = messages
    .unwrap()
    .into_iter()
    .nth(0)
    .unwrap();
  let receipt_handle = res.receiptHandle;
  let body = res.body.unwrap();

  let metadata: BlurhashQueueOutputItem = serde_json::from_str(&body).unwrap();

  println!("About to place {} in DynamoDB", metadata.name);

  dynamo_client
    .put_item()
    .table_name(table_name)
    .item("blurhash", AttributeValue::S(metadata.blurhash))
    .item("created_at", AttributeValue::N(time_stamp.to_string()))
    .item("media_type", AttributeValue::S("image".to_string()))
    .item("name", AttributeValue::S(metadata.name.to_string()))
    .item("pk", AttributeValue::S("image|widescreen_wallpaper".to_string()))
    .item("sk", AttributeValue::S(metadata.name.to_string()))
    .item("thumbnail_url", AttributeValue::S(metadata.thumbnail_url))
    .item("updated_at", AttributeValue::N(time_stamp.to_string()))
    .item("url", AttributeValue::S(metadata.url.to_string()))
    .send()
    .await
    .unwrap();

  let image = bytes::Bytes::from(Http::image_stream(metadata.url).await);

  println!("About to place {}'s stream in S3 and remove from download wallpaper queue", metadata.name);

  s3_client 
    .put_object()
    .bucket(bucket_name)
    .body(ByteStream::from(image))
    .key(metadata.name)
    .send()
    .await
    .unwrap();

  sqs_client
    .delete_message()
    .queue_url(queue_url)
    .receipt_handle(receipt_handle)
    .send()
    .await
    .unwrap();

  Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = lambda_runtime::service_fn(handler);
    lambda_runtime::run(handler).await?; 
    Ok(())
}