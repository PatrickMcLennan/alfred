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
  let auth_cookie_name = dotenv!("COLLECTOR_COOKIE_NAME").to_string();

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

  let (email, password) = (login_dto.email.unwrap_or_default(), login_dto.password.unwrap_or_default());
  if email.is_empty() | password.is_empty() { return Ok(four_hundred) }

  let cognito_client = Cognito::new().await;

  match cognito_client
    .initiate_auth()
    .auth_flow(AuthFlowType::UserPasswordAuth)
    .client_id(user_pool_client_id)
    .auth_parameters("USERNAME".to_string(), email)
    .auth_parameters("PASSWORD".to_string(), password)
    .send()
    .await {
      Ok(v) => {
        println!("Success!  Here's the output: {:?}", v);
        let credentials = v.authentication_result.unwrap();
        let access_token = credentials.access_token.unwrap_or_default();
        let refresh_token = credentials.refresh_token.unwrap_or_default();
        let id_token = credentials.id_token.unwrap_or_default();
        return Ok(HttpResponse {
          statusCode: 200,
          body: serde_json::to_string(&HttpResponseBody {
            success: true,
            message: "Logged in".to_string(),
            id_token: Some(id_token)
          }).unwrap_or_default(),
          multiValueHeaders: Some(MultiValueHeaders {
            Set_Cookie: vec![
              format!("access-token=Bearer {};httpOnly;Secure;", access_token),
              format!("refresh-token={};httpOnly;Secure;", refresh_token)
            ]
          })
        })
      },
      Err(e) => {
        println!("Error!  Here's the error: {}", e);
        return Ok(four_hundred);
      }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = lambda_runtime::service_fn(handler);
    lambda_runtime::run(handler).await?; 
    Ok(())
}