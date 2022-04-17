use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageSearchDto {
  pub limit: Option<i32>,
  pub start_key: Option<String>,
  pub contains: Option<String>
}