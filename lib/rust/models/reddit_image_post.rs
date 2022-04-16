use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct RedditImagePost {
  pub name: String,
  pub url: String,
	pub thumbnail_url: String,
}

