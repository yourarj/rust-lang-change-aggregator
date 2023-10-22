//! dto

use std::process::id;

use scraper::{Element, Html, Selector};

use crate::parser::Parse;

/// RustVersionAnnoucement
#[derive(Debug, Default)]
pub struct RustVersionAnnoucement {
  version: f32,
  uri: String,
}

impl RustVersionAnnoucement {
  /// construct new instance
  pub fn new(version: f32, uri: String) -> Self {
    Self { version, uri }
  }

  /// get version
  pub fn version(&self) -> f32 {
    self.version
  }

  /// get uri
  pub fn uri(&self) -> String {
    self.uri.to_owned()
  }
}

impl Parse<Self> for RustVersionAnnoucement {
  fn parse(input: &str) -> Vec<RustVersionAnnoucement> {
    let doc = Html::parse_document(input);

    //#posts > div > table > tbody > tr:nth-child(140) > td:nth-child(2) > a
    let link_selector =
      Selector::parse("#posts > div > table > tbody > tr > td > a")
        .expect("Invalid selector provided");

    let mut idx: usize = 1;
    for element in doc.select(&link_selector).rev() {
      if element.inner_html().starts_with("Announcing Rust ") {
        let text = element.inner_html();
        let Some(link) = element.value().attr("href") else {
          continue;
        };
        println!("Announcement: {idx:03}: {text}: {link:?}");
        idx += 1;
      }
    }

    vec![]
  }
}
