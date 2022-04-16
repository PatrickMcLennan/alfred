use reqwest::Error;
use bytes::Bytes;
pub struct Http {}

impl Http {
  pub async fn get(url: &str) -> Result<String, Error> {
    let resp = reqwest::get(url)
      .await
      .unwrap()
      .text()
      .await;

    match resp {
      Ok(s) => Ok(s),
      Err(e) => Err(e),
    }
  }

  pub async fn image_stream(url: String) -> Bytes {
    reqwest::get(url)
      .await
      .unwrap()
      .bytes()
      .await
      .unwrap()
  }
}