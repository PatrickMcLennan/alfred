use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageSearchDto {
  pub limit: Option<String>, // i32 when unwrapped
  pub start_key: Option<String>,
  pub contains: Option<String>
}