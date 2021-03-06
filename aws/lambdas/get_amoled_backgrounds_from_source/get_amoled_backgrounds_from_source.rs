#[macro_use]
extern crate dotenv_codegen;

use lambda_runtime::{LambdaEvent, Error};
use std::collections::HashMap;
use lib::models::{RedditImagePost, BlurhashQueueInputItem};
use lib::services::*;
use lib::repositories::*;
use aws_sdk_dynamodb::model::{AttributeValue, KeysAndAttributes};
use serde::Deserialize;
use aws_sdk_sqs::model::{SendMessageBatchRequestEntry};
use futures::future::join_all;
use uuid::Uuid;

#[derive(Deserialize)]
struct Event {}

async fn handler(_: LambdaEvent<Event>) -> Result<(), Error> {
  let endpoint: String = dotenv!("AMOLED_BACKGROUNDS_URL").to_string();
  let blurhash_queue_name = dotenv!("COLLECTOR_BLURHASH_QUEUE_NAME").to_string();
  let table_name = dotenv!("COLLECTOR_DYNAMODB").to_string();

  let sqs_client = SQS::new().await;
  let dynamo_client = DynamoDB::new().await;
  let get_html = Http::get(&endpoint).await.unwrap();
  
  // Get images, determine which aren't in dynamo
  let posts = HtmlParser::parse_reddit_images(get_html);
  let dynamo_get_keys: Vec<HashMap<String, AttributeValue>> = posts
    .iter()
    .map(|(name, _)| 
      HashMap::from([
        ("pk".to_string(), AttributeValue::S("image|amoled_background".to_string())),
        ("sk".to_string(), AttributeValue::S(name.to_string()))
      ])
    )
    .collect();

  let get_keys_and_attributes = KeysAndAttributes::builder()
    .set_keys(Some(dynamo_get_keys))
    .projection_expression("sk")
    .build();

  let dynamo_posts: HashMap<String, Option<()>> = dynamo_client
    .batch_get_item()
    .request_items(&table_name, get_keys_and_attributes)
    .send()
    .await
    .unwrap()
    .responses()
    .unwrap()
    .get(&table_name)
    .unwrap()
    .iter()
    .map(|hashmap| (hashmap.get("sk").unwrap().as_s().unwrap().to_string(), None))
    .collect();

  let new_posts: HashMap<String, RedditImagePost> = posts
    .into_iter()
    .filter(|(name, _)| {
      let is_duplicate = dynamo_posts.contains_key(name);
      !is_duplicate
    })
    .collect();

  if new_posts.len() == 0 { return Ok(()) }

  // Place stringified metadata of each new image on the blurhash queue in batches of 10 per AWS reqs
  let queue_url = SQS::get_queue_url(&sqs_client, blurhash_queue_name).await;
  let queue_entries: Vec<SendMessageBatchRequestEntry> = new_posts
    .iter()
    .map(|(_, post)| {
      let entry_id = Uuid::new_v4();
      let json_string = serde_json::ser::to_string(
        &BlurhashQueueInputItem {
          url: post.url.to_string(),
          thumbnail_url: post.thumbnail_url.to_string(),
          name: post.name.to_string(),
          pk: "image|amoled_background".to_string()
        }
      ).unwrap_or_default();
      println!("Inserting {} into the blurhash queue now . . .", json_string);
      SendMessageBatchRequestEntry::builder()
        .set_message_body(Some(json_string))
        .set_id(Some(entry_id.to_string()))
        .build()
    })
    .collect();

  let batched_queue_entries = queue_entries.chunks(10);
  join_all(
    batched_queue_entries
      .map(|entries_chunk|
        sqs_client
          .send_message_batch()
          .queue_url(&queue_url)
          .set_entries(Some(entries_chunk.to_vec()))
          .send()
      )
  )
    .await
    .into_iter()
    .for_each(|result| { result.unwrap(); });
  
  Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = lambda_runtime::service_fn(handler);
    lambda_runtime::run(handler).await?; 
    Ok(())
}