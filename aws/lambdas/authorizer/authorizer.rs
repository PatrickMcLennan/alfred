#[macro_use]
extern crate dotenv_codegen;

use lambda_runtime::{LambdaEvent, Error};
use lib::repositories::*;
use lib::models::*;
// use serde::{Deserialize, Serialize};
use serde::{Deserialize, Serialize};
use aws_sdk_cognitoidentityprovider::model::{AuthFlowType};
use cookie::{Cookie, CookieJar};
use jsonwebtoken::{Algorithm, decode, DecodingKey, Validation};
use jsonwebtoken::errors::ErrorKind;

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct EventHeaders {
  Cookie: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct APIGatewayCustomAuthorizerRequest {
  #[serde(rename = "type")]
  _type: String,
  headers: EventHeaders,
  method_arn: String,
}

#[derive(Serialize, Debug)]
#[allow(non_snake_case)]
pub struct HttpResponseHeaders {
  #[serde(rename(serialize = "Set-Cookie"))]
  Set_Cookie: Vec<String>,
}
#[derive(Serialize, Debug)]
#[allow(non_snake_case)]
pub struct HttpResponse {
  // pub multiValueHeaders: Option<HttpResponseHeaders>,
  pub body: PolicyDocument,
}

async fn handler(event: LambdaEvent<APIGatewayCustomAuthorizerRequest>) -> Result<PolicyDocument, Error> {
  let user_pool_client_id = dotenv!("COLLECTOR_USER_POOL_CLIENT_ID").to_string();
  let user_pool_id = dotenv!("COLLECTOR_USER_POOL_ID").to_string();
  let jwt_n = dotenv!("JWT_N").to_string();
  let jwt_e = dotenv!("JWT_E").to_string();
  let method_arn = event.payload.method_arn;
  let cognito_client = Cognito::new().await;
  let mut new_access_token_string = String::new();

  let cookie_string = match event.payload.headers.Cookie {
    Some(v) => v,
    None => {
      println!("No event.payload.Cookie found");
      return Ok(generate_document("Unauthorized", None, "Deny", &method_arn))
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

  let access_token_string = cookie_jar
    .get("alfred_access_token")
    .map(|v| v.value())
    .unwrap_or_default();

  let refresh_token_string = cookie_jar
    .get("alfred_refresh_token")
    .map(|v| v.value())
    .unwrap_or_default();

  if access_token_string.is_empty() || refresh_token_string.is_empty() { 
    return Ok(generate_document("", None, "Deny", &method_arn))
  }

  let decoder = match DecodingKey::from_rsa_components(&jwt_n, &jwt_e) {
    Ok(d) => d,
    Err(e) => {
      println!("Error creating the decoder: {}", e);
      return Ok(generate_document("", None, "Deny", &method_arn))
    }
  };

  let access_token = match decode::<AccessTokenPayload>(
    &access_token_string,
    &decoder,
    &Validation::new(Algorithm::RS256)
  ) {
    Ok(v) => v,
    Err(e) => {
      let needs_refresh = e.kind() == &ErrorKind::ExpiredSignature;
      if !needs_refresh {
        println!("Error getting the access token and it doesn't need a refresh: {}", e);
        return Ok(generate_document("", None, "Deny", &method_arn))
      };
      
      match &cognito_client
        .initiate_auth()
        .auth_flow(AuthFlowType::RefreshTokenAuth)
        .client_id(&user_pool_client_id)
        .auth_parameters("REFRESH_TOKEN", refresh_token_string)
        .send()
        .await {
          Ok(new_v) => {
            let credentials = new_v.authentication_result.as_ref().unwrap();
            let new_token_string = credentials.access_token.as_ref().unwrap();
            new_access_token_string = new_token_string.to_string();

            decode::<AccessTokenPayload>(
              &new_token_string, 
              &decoder, 
              &Validation::new(Algorithm::RS256)
            ).unwrap()
          },
          Err(e) => {
            println!("Error refreshing the token, details here: {}", e);
            return Ok(generate_document("", None, "Deny", &method_arn))
          }
        }
    }
  };

  let user_id = access_token.claims.sub;

  if access_token.claims.client_id != user_pool_client_id {
    return Ok(generate_document("", None, "Deny", &method_arn))
  }

  if !access_token.claims.iss.contains(&user_pool_id.to_string()) {
    return Ok(generate_document("", None, "Deny", &method_arn))
  }

  if access_token.claims.token_use != "access" {
    return Ok(generate_document("", None, "Deny", &method_arn))
  }

  let new_cookie_string = if new_access_token_string.is_empty() { 
    access_token_string.to_string() 
  } else { 
    new_access_token_string.to_string() 
  };

  let success_document = generate_document(
    &user_id, 
    Some(AuthorizerContext { 
      Set_Cookie: format!("alfred_access_token={};httpOnly;Secure;SameSite=None;", new_cookie_string) 
    }),
    "Allow", 
    &method_arn
  );
  Ok(success_document)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = lambda_runtime::service_fn(handler);
    lambda_runtime::run(handler).await?; 
    Ok(())
}