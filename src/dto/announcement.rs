//! dto

use scraper::{Html, Selector};

use crate::parser::Parse;

/// RustVersionAnnouncement
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct RustVersionAnnouncement {
  seq: usize,
  version: String,
  uri: String,
}

impl RustVersionAnnouncement {
  /// construct new instance
  pub fn new(version: String, uri: String) -> Self {
    Self {
      seq: 1,
      version,
      uri,
    }
  }

  /// get seq
  pub fn seq(&self) -> usize {
    self.seq
  }

  /// get version
  pub fn version(&self) -> &str {
    &self.version
  }

  /// get uri
  pub fn uri(&self) -> &str {
    &self.uri
  }
}

impl Parse<Self> for RustVersionAnnouncement {
  fn parse(input: &str) -> Vec<RustVersionAnnouncement> {
    let doc = Html::parse_document(input);

    //#posts > div > table > tbody > tr:nth-child(140) > td:nth-child(2) > a
    let link_selector =
      Selector::parse("#posts > div > table > tbody > tr > td > a")
        .expect("Invalid selector provided");

    let mut annoucements = vec![];
    let mut idx: usize = 1;

    for element in doc.select(&link_selector).rev() {
      if element.inner_html().starts_with("Announcing Rust ") {
        let mut text = element.inner_html();
        let Some(uri) = element.value().attr("href") else {
          continue;
        };
        annoucements.push(Self {
          seq: idx,
          version: text.split_off(16),
          uri: uri.to_owned(),
        });
        idx += 1;
      }
    }

    annoucements
  }
}
