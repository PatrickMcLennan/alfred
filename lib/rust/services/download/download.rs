use std::{fs::{File}, io::Write};

pub struct Download {}

impl Download {
  pub async fn image(dir: String, url: String, name: String) -> () {
    let mut file = File::create(format!("{}/{}", dir, name)).unwrap();
    let bytes = reqwest::get(url).await.unwrap().bytes().await.unwrap();
    file.write_all(&bytes).unwrap()
  }
}