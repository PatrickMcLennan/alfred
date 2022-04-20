use serde::Deserialize;

pub struct AccessTokenHeader {
  pub kid: String,
  pub alg: String,
}

#[derive(Deserialize)]
pub struct AccessTokenPayload {
  pub origin_jti: String,
  pub sub: String,
  pub event_id: String,
  pub token_use: String,
  pub scope: String,
  pub auth_time: u64,
  pub iss: String,
  pub exp: u64,
  pub iat: u64,
  pub jti: String,
  pub client_id: String,
  pub username: String
}