use scraper::{Html, Selector};
use crate::models::{MovieCollection, RedditImagePost};
use std::collections::HashMap;

pub struct HtmlParser {}

impl HtmlParser {

  pub fn parse_movie_collections(string_http_response: String) -> HashMap<String, MovieCollection> {
    // println!("Beginning to parse string_http_response in HtmlParser: {}", string_http_response);
    let document = Html::parse_document(&string_http_response);
    let entry_selector = Selector::parse(".list-entry").unwrap();
    let elements = document.select(&entry_selector);
    let mut posts: HashMap<String, MovieCollection> = HashMap::new();

    println!("string_http_response: {:?}", string_http_response);
    println!("document: {:?}", document);
    println!("elements: {:?}", elements);

    for post in document.select(&entry_selector) {

      println!("Beginning to parse post: {:?}", post);

      let name = match post
        .select(&Selector::parse(".item-title > a").unwrap())
        .nth(0) {
          Some(v) => v.text().collect::<String>(),
          None => {
            println!("post has no name: {:?}", post);
            continue
          }
        };

      let uploaded_at = match post
        .select(&Selector::parse(".item-uploaded > label").unwrap())
        .nth(0) {
          Some(v) => v.text().collect::<String>(),
          None => {
            println!("post has no uploaded_at: {:?}", post);
            continue
          }
        };

      let size_string = match post
        .select(&Selector::parse(".item-size").unwrap())
        .nth(0) {
          Some(v) => v.text().collect::<String>(),
          None => {
            println!("post has no size_string: {:?}", post);
            continue
          }
        };

      let size_bytes = match post
        .select(&Selector::parse(".item-size > input").unwrap())
        .nth(0) {
          Some(v) => v
            .text()
            .collect::<String>()
            .parse::<u64>()
            .unwrap_or_default(),
          None => {
            println!("post has no size_bytes: {:?}", post);
            continue
          }
        };

      let magnet_url = match post 
        .select(&Selector::parse(".item-icons > a").unwrap())
        .nth(0) {
          Some(v) => v.value().attr("href").unwrap_or_default().to_string(),
          None => {
            println!("post has no magbet_url: {:?}", post);
            continue
          }
        };

      posts.insert(name.to_string(), MovieCollection {
        name, 
        uploaded_at,
        magnet_url,
        size_string,
        size_bytes
      });
    };

    posts
  }
  pub fn parse_reddit_images(string_http_response: String) -> HashMap<String, RedditImagePost> {
    let document = Html::parse_document(&string_http_response);
    let post_selector = Selector::parse("div.thing").unwrap(); 
    let thumbnail_selector = Selector::parse("a.thumbnail[data-event-action='thumbnail'] > img").unwrap();
    let mut posts: HashMap<String, RedditImagePost> = HashMap::new();

    for post in document.select(&post_selector) {

      match post.value().attr("data-nsfw") {
          Some(nsfw) => if nsfw == "true" { continue; },
          None => continue,
      };

      match post.value().attr("data-promoted") {
          Some(promoted) => if promoted == "true" { continue; },
          None => continue,
      };

      let url = match post.value().attr("data-url") {
          Some(url) => {
              if !url.contains("jpg") && !url.contains("jpeg") && !url.contains("png") {
                  continue;
              } else {
                  url
              }
          },
          None => continue,
      };

      let thumbnail_img = post.select(&thumbnail_selector);
      let mut thumbnail_url = String::new();
      for thumbnail in thumbnail_img {
        thumbnail_url = match thumbnail.value().attr("src") {
          Some(s) => format!("https://{}", s),
          None => continue
        }
      }

      if thumbnail_url.len() == 0 { continue; }

      let name_split: Vec<&str> = url.split(".").collect();
      let name_without_ext = name_split[name_split.len() - 2].replace("it/", "");
      let ext_split: Vec<&str> = url.split(".").collect();
      let ext = ext_split[ext_split.len() - 1];
      let name = format!("{}.{}", name_without_ext, ext);
      let url = String::from(url);

      posts.insert(name.to_string(), RedditImagePost { name, url, thumbnail_url });
    };
    posts
  }
}