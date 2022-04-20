#[macro_use]
extern crate dotenv_codegen;

use serde::{Deserialize, Serialize};
use lambda_runtime::{LambdaEvent, Error};
use lib::repositories::*;
use aws_sdk_cognitoidentityprovider::model::{AuthFlowType};

#[derive(Serialize, Debug)]
#[allow(non_snake_case)]
pub struct MultiValueHeaders {
  #[serde(rename(serialize = "Set-Cookie"))]
  Set_Cookie: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct LoginDto {
  pub email: Option<String>,
  pub password: Option<String>
}

#[derive(Deserialize, Debug)]
struct HttpEvent {
  pub body: Option<String>
}

#[derive(Serialize)]
pub struct HttpResponseBody {
  pub success: bool,
  pub message: String,
  pub id_token: Option<String>
}

#[derive(Serialize)]
#[allow(non_snake_case)]
struct HttpResponse {
  pub statusCode: u16,
  pub body: String,
  pub multiValueHeaders: Option<MultiValueHeaders>
}

async fn handler(event: LambdaEvent<HttpEvent>) -> Result<HttpResponse, Error> {
  let user_pool_client_id = dotenv!("COLLECTOR_USER_POOL_CLIENT_ID").to_string();
  let four_hundred = HttpResponse {
    statusCode: 400,
    body: serde_json::to_string(&HttpResponseBody {
      success: false,
      message: "incorrect params".to_string(),
      id_token: None
    }).unwrap_or_default(),
    multiValueHeaders: None
  };


  let body = event.payload.body.unwrap_or_default();
  if body.is_empty() { return Ok(four_hundred) };
  let login_dto: LoginDto = match serde_json::from_str(&body) {
    Ok(v) => v,
    Err(_) => return Ok(four_hundred)
  };

  let (email, password) = (
    match login_dto.email {
      Some(v) => if v.is_empty() { return Ok(four_hundred) } else { v },
      None => return Ok(four_hundred)
    }, 
    match login_dto.password {
      Some(v) => if v.is_empty() { return Ok(four_hundred) } else { v },
      None => return Ok(four_hundred)
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
        
        return Ok(HttpResponse {
          statusCode: 200,
          body: serde_json::to_string(&HttpResponseBody {
            success: true,
            message: "Logged in".to_string(),
            id_token: Some(id_token.to_string())
          }).unwrap_or_default(),
          multiValueHeaders: Some(MultiValueHeaders {
            Set_Cookie: vec![
              format!("alfred_access_token={};httpOnly;Secure;", access_token),
              format!("alfred_expires_in={};httpOnly;Secure;", expires_in),
              format!("alfred_id_token={};httpOnly;Secure;", id_token),
              format!("alfred_refresh_token={};httpOnly;Secure;", refresh_token),
              format!("alfred_token_type={};httpOnly;Secure;", token_type)
            ]
          })
        })
      },
      Err(e) => {
        println!("Error!  Here's the error: {}", e);
        return Ok(HttpResponse {
          statusCode: 401,
          body: "Unauthorized".to_string(),
          multiValueHeaders: None,
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