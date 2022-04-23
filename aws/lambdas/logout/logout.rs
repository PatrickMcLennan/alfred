#[macro_use]
extern crate dotenv_codegen;

use serde::{Deserialize, Serialize};
use lambda_runtime::{LambdaEvent, Error};
use lib::repositories::*;
use lib::models::{Response, ResponseHeaders};
use cookie::{Cookie, CookieJar};
use chrono::TimeZone;

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RequestHeaders {
  Cookie: Option<String>,
}

#[derive(Deserialize, Debug)]
struct RequestEvent {
  pub headers: Option<RequestHeaders>
}

async fn handler(event: LambdaEvent<RequestEvent>) -> Result<Response, Response> {
  println!("event: {:?}", event);
  let user_pool_client_id = dotenv!("COLLECTOR_USER_POOL_CLIENT_ID").to_string();

  let invalid_attempt = Response {
    statusCode: 400,
    body: "Invalid parameters".to_string(),
    multiValueHeaders: None,
    headers: None
  };

  let cookie_string = match event
    .payload
    .headers
    .unwrap_or_else(|| RequestHeaders { Cookie: None })
    .Cookie {
      Some(v) => v,
      None => {
        println!("No event.payload.Cookie found");
        return Err(invalid_attempt)
      }
    };

  let mut cookie_jar = CookieJar::new();
  cookie_string
    .split(';')
    .collect::<Vec<&str>>()
    .iter()
    .for_each(|cookie| { 
      cookie_jar.add_original(Cookie::parse(cookie.to_string()).unwrap()); 
    });

  match Cognito::new()
    .await
    .revoke_token()
    .token(cookie_jar
      .get("alfred_refresh_token")
      .map(|v| v.value())
      .unwrap_or_default()
    )
    .client_id(user_pool_client_id)
    .send()
    .await {
      Ok(_) => (),
      Err(e) => {
        println!("Token wasn't revoked, error: {}", e);
        return Err(invalid_attempt)
      }
    }

  let expired_date = chrono::Local.ymd(1970, 1, 1).to_string();
  let cookie_defaults = "httpOnly;Path=/;Secure";

  Ok(Response {
    statusCode: 201,
    body: "Successfully logged out.".to_string(),
    multiValueHeaders: Some(ResponseHeaders {
      Set_Cookie: vec![
        format!("alfred_access_token={};{};Expires={}", String::new(), cookie_defaults, expired_date),
        format!("alfred_expires_in={};{};Expires={}", String::new(), cookie_defaults, expired_date),
        format!("alfred_id_token={};{};Expires={}", String::new(), cookie_defaults, expired_date),
        format!("alfred_refresh_token={};{};Expires={}", String::new(), cookie_defaults, expired_date),
        format!("alfred_token_type={};{};Expires={}", String::new(), cookie_defaults, expired_date),
        format!("alfred_is_logged_in={};Path=/;Secure;Expires={}", "false".to_string(), expired_date)
      ]
    }),
    headers: None
  })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
  let handler = lambda_runtime::service_fn(handler);
  lambda_runtime::run(handler).await?; 
  Ok(())
}