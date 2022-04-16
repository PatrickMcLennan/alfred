   
#[macro_use]
extern crate dotenv_codegen;

use std::fs;
use std::path::Path;
use lib::services::*;
use futures::future::join_all;

#[tokio::main]
async fn main() -> () {
  let wallpaper_dir_string = dotenv!("WIDESCREEN_WALLPAPERS_DIR").to_string();
  let endpoint = dotenv!("WIDESCREEN_WALLPAPERS_URL").to_string();

  let html = Http::get(&endpoint).await.unwrap();
  let mut posts = HtmlParser::parse_reddit_images(html);
  let existing_files = fs::read_dir(&wallpaper_dir_string)
    .unwrap()
    .map(|f| f.unwrap().path());

  for file in existing_files {
    let name = Path::new(&file).file_name().unwrap().to_str().unwrap();
    if posts.contains_key(name) { 
      posts.remove(name);
      ()
    };
  };

  println!("Starting to download {} images . . .", posts.len());

  let futures = posts
    .iter()
    .map(|(name, image)| tokio::task::spawn(Download::image(wallpaper_dir_string.clone(), image.url.clone(), name.clone())));

  join_all(futures).await;

  return ();
}