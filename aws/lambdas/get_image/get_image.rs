#[macro_use]
extern crate dotenv_codegen;
use serde::{Deserialize, Serialize};
use aws_sdk_dynamodb::model::AttributeValue;

use lambda_runtime::{LambdaEvent, Error};
use lib::models::*;
use lib::repositories::*;

#[derive(Serialize)]
pub struct ResponseBody {
    pub wallpaper: Option<DynamoImage>,
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct QueryStringParameters {
    pub sk: Option<String>,
    pub pk: Option<String>
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct RequestEvent {
    pub queryStringParameters: Option<QueryStringParameters>
}

async fn handler(event: LambdaEvent<RequestEvent>) -> Result<Response, Response> {
    let four_hundred = Response {
      statusCode: 400,
      body: String::new(),
      headers: None,
      multiValueHeaders: None,
    };
    
    let query_string = match event.payload.queryStringParameters {
      Some(v) => v,
      None => return Err(four_hundred)
    };
    
    let sk = match query_string.sk {
      Some(s) => if s.is_empty() { return Err(four_hundred) } else { s },
      None => return Err(four_hundred)
    };
    
    let pk = match query_string.pk {
      Some(s) => if s.is_empty() { return Err(four_hundred) } else { s },
      None => return Err(four_hundred)
    };

    let result = match DynamoDB::new()
      .await
      .query()
      .table_name(dotenv!("COLLECTOR_DYNAMODB").to_string())
      .key_condition_expression("#pk = :pk and #sk = :sk")
      .expression_attribute_names("#pk", "pk")
      .expression_attribute_names("#sk", "sk")
      .expression_attribute_values(":pk", AttributeValue::S(pk.to_string()))
      .expression_attribute_values(":sk", AttributeValue::S(sk))
      .scan_index_forward(false)
      .send()
      .await {
        Ok(v) => v,
        Err(e) => {
          println!("Error in result: {:?}", e);
          return Err(four_hundred)
        }
      };

    if result.count <= 0 {
      return Err(Response {
        statusCode: 404,
        body: serde_json::to_string(&ResponseBody {
          message: "No items found with that criteria".to_string(),
          wallpaper: None
        }).unwrap_or_default(),
        multiValueHeaders: None,
        headers: None
      })
    }

    let items = match result.items {
      Some(v) => v,
      None => return Err(four_hundred)
    };

    println!("items: {:?}", items);

    let wallpaper = items
      .iter()
      .map(|hashmap| 
        DynamoImage {
          blurhash: hashmap.get("blurhash").unwrap().as_s().unwrap().to_string(),
          created_at: hashmap.get("created_at").unwrap().as_n().unwrap().parse::<u64>().unwrap(),
          ignored: *hashmap.get("ignored").unwrap().as_bool().unwrap(),
          media_type: hashmap.get("media_type").unwrap().as_s().unwrap().to_string(),
          name: hashmap.get("name").unwrap().as_s().unwrap().to_string(),
          pk: hashmap.get("pk").unwrap().as_s().unwrap().to_string(),
          sk: hashmap.get("sk").unwrap().as_s().unwrap().to_string(),
          thumbnail_url: hashmap.get("thumbnail_url").unwrap().as_s().unwrap().to_string(),
          updated_at: hashmap.get("created_at").unwrap().as_n().unwrap().parse::<u64>().unwrap(),
          url: hashmap.get("url").unwrap().as_s().unwrap().to_string(),
        }
      )
      .collect::<Vec<DynamoImage>>()
      .into_iter()
      .nth(0)
      .unwrap();

    println!("wallpaper: {:?}", wallpaper);

    Ok(Response {
      statusCode: 200,
      body: serde_json::to_string(&ResponseBody {
        message: "Image found".to_string(),
        wallpaper: Some(wallpaper)
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