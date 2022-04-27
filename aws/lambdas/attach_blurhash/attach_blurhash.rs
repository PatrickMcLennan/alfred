#[macro_use]
extern crate dotenv_codegen;

use lambda_runtime::{LambdaEvent, Error};
use serde::Deserialize;
use lib::models::{BlurhashQueueInputItem, BlurhashQueueOutputItem};
use lib::repositories::*;
use lib::services::*;
use blurhash::encode;
use futures::future::join_all;

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
  let blurhash_queue_name = dotenv!("COLLECTOR_BLURHASH_QUEUE_NAME").to_string();
  let download_wallpaper_queue_name = dotenv!("COLLECTOR_DOWNLOAD_WALLPAPER_QUEUE_NAME").to_string();

  let sqs_client = SQS::new().await;
  let futures = join_all([
    SQS::get_queue_url(&sqs_client, download_wallpaper_queue_name), 
    SQS::get_queue_url(&sqs_client, blurhash_queue_name.to_string())]
  ).await;
  let (blurhash_queue_url, download_wallpaper_queue_url) = (&futures[1], &futures[0]);

  // Get stringified queue message, create blurhash from thumbnailUrl metadata
  let messages = event.payload.Records;
  let res = messages
    .unwrap()
    .into_iter()
    .nth(0)
    .unwrap();
  let reciept_handle = res.receiptHandle;
  let body = res.body.unwrap();

  let input_item: BlurhashQueueInputItem = serde_json::from_str(&body).unwrap();

  let stream = Http::image_stream(input_item.thumbnail_url.to_string()).await;
  let image = bytes::Bytes::from(stream);
  let memory_image = image::load_from_memory(&image).unwrap();
  let blurhash = encode(4,3, 50, 50, &memory_image.to_rgb8().to_vec());
  let output_raw = BlurhashQueueOutputItem {
    url: input_item.url,
    thumbnail_url: input_item.thumbnail_url,
    name: input_item.name,
    blurhash,
    pk: input_item.pk
  };

  // Stringify new metatdata with blurhash, place onto next queue, remove from calling queue
  let output_string = serde_json::ser::to_string(&output_raw).unwrap();

  println!("Placing {} on the download wallpaper queue with a blurhash, removing entry from blurhash queue", output_string);

  sqs_client
    .send_message()
    .queue_url(download_wallpaper_queue_url)
    .message_body(output_string)
    .send()
    .await
    .unwrap();

  sqs_client  
    .delete_message()
    .queue_url(blurhash_queue_url)
    .receipt_handle(reciept_handle)
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