use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MovieCollection {
  pub name: String,
  pub uploaded_at: String,
  pub magnet_url: String,
  pub size_string: String,
  pub size_bytes: u64,
}