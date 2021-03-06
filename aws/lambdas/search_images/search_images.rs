#[macro_use]
extern crate dotenv_codegen;
use serde::{Deserialize, Serialize};
use aws_sdk_dynamodb::model::AttributeValue;

use lambda_runtime::{LambdaEvent, Error};
use lib::models::*;
use lib::repositories::*;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct RequestEvent {
  pub body: Option<String>
}

#[derive(Serialize)]
pub struct ResponseBody {
  pub total: i32,
  pub images: Vec<DynamoImage>,
  pub message: String,
}

async fn handler(event: LambdaEvent<RequestEvent>) -> Result<Response, Response> {
  let table_name = dotenv!("COLLECTOR_DYNAMODB").to_string();

  let four_hundred = Response {
    statusCode: 400,
    body: serde_json::to_string(&ResponseBody {
      total: 0,
      message: "Invalid params".to_string(),
      images: vec![]
    }).unwrap_or_default(),
    multiValueHeaders: None,
    headers: None
  };

  let body = event.payload.body.unwrap_or_default();

  let dynamo_client = DynamoDB::new().await;

  let mut limit = 0 as i32;
  let mut start_key = String::new();
  let mut contains = String::new();
  let mut pk = String::new();

  if !body.is_empty() {
    let image_search_dto: ImageSearchDto = serde_json::from_str(&body).unwrap();
    match image_search_dto.pk {
      Some(v) => pk = v,
      None => ()
    }
    match image_search_dto.contains { 
      Some(v) => contains = v, 
      None => () 
    };
    match image_search_dto.limit { 
      Some(v) => limit = v, 
      None => () 
    };
    match image_search_dto.start_key { 
      Some(v) => start_key = v, 
      None => () 
    };
  } else {
    return Err(four_hundred)
  }

  if pk.is_empty() { return Err(four_hundred) }
  
  let mut results_query = dynamo_client
    .query()
    .table_name(table_name)
    .key_condition_expression("#pk = :pk")
    .expression_attribute_names("#pk", "pk")
    .expression_attribute_values(":pk", AttributeValue::S(pk))
    .scan_index_forward(false);

  if !contains.is_empty() {
    results_query = results_query
      .set_filter_expression(Some("contains(#name, :name) and #ignored = :false".to_string()))
      .expression_attribute_names("#name", "name")
      .expression_attribute_names("#ignored", "ignored")
      .expression_attribute_values(":name", AttributeValue::S(contains))
      .expression_attribute_values(":false", AttributeValue::Bool(false));
    } else {
    results_query = results_query
      .set_filter_expression(Some("#ignored = :false".to_string()))
      .expression_attribute_names("#ignored", "ignored")
      .expression_attribute_values(":false", AttributeValue::Bool(false));
  }
  if limit >= 1 { 
    results_query = results_query.set_limit(Some(limit)); 
    () 
  }
  if !start_key.is_empty() {
    let map = HashMap::from([
      ("pk".to_string(), AttributeValue::S("image|widescreen_wallpaper".to_string())),
      ("sk".to_string(), AttributeValue::S(start_key)),
    ]);
    results_query = results_query.set_exclusive_start_key(Some(map))
  }

  let results = results_query
    .send()
    .await
    .unwrap();

  if results.count <= 0 {
    return Err(Response {
      statusCode: 404,
      body: serde_json::to_string(&ResponseBody {
        total: 0,
        message: "No items found with that criteria".to_string(),
        images: vec![]
      }).unwrap_or_default(),
      multiValueHeaders: None,
      headers: None
    })
  }

  let formatted_items: Vec<DynamoImage> = results
    .items
    .as_ref()
    .unwrap()
    .iter()
    .map(|hashmap| 
      DynamoImage {
        blurhash: hashmap.get("blurhash").unwrap().as_s().unwrap().to_string(),
        created_at: hashmap.get("created_at").unwrap().as_n().unwrap().parse::<u64>().unwrap(),
        media_type: hashmap.get("media_type").unwrap().as_s().unwrap().to_string(),
        name: hashmap.get("name").unwrap().as_s().unwrap().to_string(),
        pk: hashmap.get("pk").unwrap().as_s().unwrap().to_string(),
        sk: hashmap.get("sk").unwrap().as_s().unwrap().to_string(),
        ignored: *hashmap.get("ignored").unwrap().as_bool().unwrap(),
        thumbnail_url: hashmap.get("thumbnail_url").unwrap().as_s().unwrap().to_string(),
        updated_at: hashmap.get("created_at").unwrap().as_n().unwrap().parse::<u64>().unwrap(),
        url: hashmap.get("url").unwrap().as_s().unwrap().to_string(),
      }
    )
    .collect();

  return Ok(Response {
    statusCode: 200,
    body: serde_json::to_string(&ResponseBody {
      total: results.count,
      images: formatted_items,
      message: format!("{} images found.", results.count),
    }).unwrap_or_default(),
    headers: None,
    multiValueHeaders: None
  })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
  let handler = lambda_runtime::service_fn(handler);
  lambda_runtime::run(handler).await?; 
  Ok(())
}