
use serde::Deserialize;

pub struct IdTokenHeader {
  pub kid: String,
  pub alg: String,
}

#[derive(Deserialize)]
pub struct IdTokenPayload {
  pub sub: String,
  pub email_verified: bool,
  pub iss: String,
  pub phone_number_verified: bool,
  // pub coognito_username: String,
  pub given_name: String,
  pub origin_jti: String,
  pub aud: String,
  pub event_id: String,
  pub token_use: String,
  pub auth_time: u64,
  pub phone_number: String,
  pub exp: u64,
  pub iat: u64,
  pub family_name: String,
  pub jti: String,
  pub email: String,
  // pub custom:isAdmin: String,
}