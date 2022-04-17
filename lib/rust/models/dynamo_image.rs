use serde::{Serialize};

#[derive(Debug, Serialize)]
pub struct DynamoImage {
  pub blurhash: String,
  pub created_at: u64,
  pub media_type: String,
  pub name: String,
  pub pk: String,
  pub sk: String,
  pub thumbnail_url: String,
  pub updated_at: u64,
  pub url: String,
}