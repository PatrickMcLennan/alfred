#[macro_use]
extern crate dotenv_codegen;
use serde::Deserialize;
use lambda_runtime::{LambdaEvent, Error};
use aws_sdk_dynamodb::model::AttributeValue;
use lib::models::*;
use lib::repositories::*;

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct DeleteImageDto {
  pub sk: Option<String>,
  pub ignored: Option<bool>
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct RequestEvent {
  pub body: Option<String>
}

async fn handler(event: LambdaEvent<RequestEvent>) -> Result<Response, Response> {
  println!("Event: {:?}", event);
  let four_hundred = Response {
    statusCode: 400,
    body: "Invalid params".to_string(),
    headers: None,
    multiValueHeaders: None
  };
  let body = match event.payload.body {
    Some(v) => if v.is_empty() { return Err(four_hundred) } else { v },
    None =>  return Err(four_hundred)
  };
  let dto: DeleteImageDto = match serde_json::from_str(&body) {
    Ok(v) => v,
    Err(_) => return Err(four_hundred)
  };
  let sk = match dto.sk {
    Some(v) => if v.is_empty() { return Err(four_hundred) } else { v },
    None => return Err(four_hundred)
  };
  let ignored = match dto.ignored {
    Some(v) => v,
    None => return Err(four_hundred)
  };

  match DynamoDB::new()
    .await
    .update_item()
    .table_name(dotenv!("COLLECTOR_DYNAMODB").to_string())
    .key("pk", AttributeValue::S("image|widescreen_wallpaper".to_string()))
    .key("sk", AttributeValue::S(sk.to_string()))
    .update_expression("SET #ignored = :ignored")
    .condition_expression("attribute_exists(pk)")
    .expression_attribute_names("#ignored", "ignored".to_string())
    .expression_attribute_values(":ignored", AttributeValue::Bool(ignored))
    .send()
    .await {
      Ok(v) => v,
      Err(e) => {
        println!("Error updating the Dynamo item: {:?}", e);
        return Err(four_hundred)
      }
    };

  let is_ignored = ignored == true;
  if is_ignored {
    match S3::new()
      .await
      .delete_object()
      .bucket(dotenv!("WIDESCREEN_WALLPAPERS_BUCKET_NAME").to_string())
      .key(sk)
      .send()
      .await {
        Ok(v) => v,
        Err(e) => {
          println!("Error deleting from bucket: {:?}", e);
          return Err(four_hundred);
        }
      };
    ()
  }

  Ok(Response {
    statusCode: 204,
    body: if is_ignored { "Image added to ignore list".to_string() } else { "Image no longer ignored".to_string() },
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