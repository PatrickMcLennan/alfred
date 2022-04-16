use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct BlurhashQueueInputItem {
  pub name: String,
  pub url: String,
  pub thumbnail_url: String,  
}

#[derive(Serialize, Deserialize)]
pub struct BlurhashQueueOutputItem {
  pub name: String,
  pub url: String,
  pub thumbnail_url: String,  
  pub blurhash: String,  
}