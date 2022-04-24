#[macro_use]
extern crate dotenv_codegen;

use serde::{Deserialize, Serialize};
use lambda_runtime::{LambdaEvent, Error};
use lib::repositories::*;
use lib::models::{Response, ResponseHeaders};
use aws_sdk_cognitoidentityprovider::model::{AuthFlowType};

#[derive(Deserialize, Debug)]
pub struct LoginDto {
  pub email: Option<String>,
  pub password: Option<String>
}

#[derive(Deserialize, Debug)]
struct RequestEvent {
  pub body: Option<String>
}

#[derive(Serialize)]
pub struct ResponseBody {
  pub success: bool,
  pub message: String,
  pub id_token: Option<String>
}

async fn handler(event: LambdaEvent<RequestEvent>) -> Result<Response, Response> {
  let user_pool_client_id = dotenv!("COLLECTOR_USER_POOL_CLIENT_ID").to_string();
  let four_hundred = Response {
    statusCode: 400,
    body: serde_json::to_string(&ResponseBody {
      success: false,
      message: "incorrect params".to_string(),
      id_token: None
    }).unwrap_or_default(),
    multiValueHeaders: None,
    headers: None
  };

  let body = event.payload.body.unwrap_or_default();
  if body.is_empty() { return Ok(four_hundred) };
  let login_dto: LoginDto = match serde_json::from_str(&body) {
    Ok(v) => v,
    Err(_) => return Err(four_hundred)
  };

  let (email, password) = (
    match login_dto.email {
      Some(v) => if v.is_empty() { return Err(four_hundred) } else { v },
      None => return Err(four_hundred)
    }, 
    match login_dto.password {
      Some(v) => if v.is_empty() { return Err(four_hundred) } else { v },
      None => return Err(four_hundred)
    }
  );

  match Cognito::new()
    .await
    .initiate_auth()
    .auth_flow(AuthFlowType::UserPasswordAuth)
    .client_id(user_pool_client_id)
    .auth_parameters("USERNAME".to_string(), email)
    .auth_parameters("PASSWORD".to_string(), password)
    .send()
    .await {
      Ok(v) => {
        let credentials = v.authentication_result.unwrap();
        let access_token = credentials.access_token.unwrap_or_default();
        let expires_in = credentials.expires_in.to_string();
        let id_token = credentials.id_token.unwrap_or_default();
        let refresh_token = credentials.refresh_token.unwrap_or_default();
        let token_type = credentials.token_type.unwrap_or_default();
        
        return Ok(Response {
          statusCode: 200,
          body: serde_json::to_string(&ResponseBody {
            success: true,
            message: "Logged in".to_string(),
            id_token: Some(id_token.to_string())
          }).unwrap_or_default(),
          multiValueHeaders: Some(ResponseHeaders {
            Set_Cookie: vec![
              format!("alfred_access_token={};httpOnly;Path=/;Secure;", access_token),
              format!("alfred_expires_in={};httpOnly;Path=/;Secure;", expires_in),
              format!("alfred_id_token={};httpOnly;Path=/;Secure;", id_token),
              format!("alfred_refresh_token={};httpOnly;Path=/;Secure;", refresh_token),
              format!("alfred_token_type={};httpOnly;Path=/;Secure;", token_type),
              format!("alfred_is_logged_in={};Path=/;Secure;", "true".to_string())
            ]
          }),
          headers: None
        })
      },
      Err(e) => {
        println!("Error!  Here's the error: {}", e);
        return Err(Response {
          statusCode: 401,
          body: "Unauthorized".to_string(),
          multiValueHeaders: None,
          headers: None
        });
      }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = lambda_runtime::service_fn(handler);
    lambda_runtime::run(handler).await?; 
    Ok(())
}