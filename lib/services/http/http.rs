use reqwest::Error;

pub struct Http {}

impl Http {
  pub async fn get(url: &str) -> Result<String, Error> {
    let resp = reqwest::get(url)
      .await
      .unwrap()
      .text();

    match resp.await {
      Ok(s) => Ok(s),
      Err(e) => Err(e),
    }
  }
}