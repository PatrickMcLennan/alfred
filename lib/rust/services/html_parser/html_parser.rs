use scraper::{Html, Selector};
use crate::models::RedditImagePost;
use std::collections::HashMap;

pub struct HtmlParser {}

impl HtmlParser {

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