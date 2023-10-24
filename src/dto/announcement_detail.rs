//! dto

use scraper::{Html, Selector};

use crate::parser::Parse;

/// RustVersionAnnoucement
#[derive(Debug, Default)]
pub struct AnnoucementDetail {
  desc: String,
  uri: String,
}

impl AnnoucementDetail {
  /// construct new instance
  pub fn new(desc: String, uri: String) -> Self {
    Self { desc, uri }
  }

  /// get description
  pub fn desc(&self) -> &str {
    &self.desc
  }

  /// get uri
  pub fn uri(&self) -> &str {
    &self.uri
  }
}

impl Parse<Self> for AnnoucementDetail {
  fn parse(input: &str) -> Vec<AnnoucementDetail> {
    let doc = Html::parse_document(input);

    //#posts > div > table > tbody > tr:nth-child(140) > td:nth-child(2) > a

    let mut announcement_detail = vec![];

    let link_selector = Selector::parse("section > div > div.post > h2")
      .expect("Invalid selector provided");
    fun_name(&doc, link_selector, &mut announcement_detail);

    let link_selector = Selector::parse("section > div > div.post > h3")
      .expect("Invalid selector provided");
    fun_name(&doc, link_selector, &mut announcement_detail);

    announcement_detail
  }
}

fn fun_name(
  doc: &Html,
  link_selector: Selector,
  announcement_detail: &mut Vec<AnnoucementDetail>,
) {
  for element in doc.select(&link_selector) {
    let mut children = element.children();
    let Some(link) = children
      .next()
      .and_then(|e| e.value().as_element())
      .and_then(|elem| elem.attr("href"))
    else {
      continue;
    };

    let Some(text) = children.next().and_then(|e| e.value().as_text()) else {
      continue;
    };
    announcement_detail.push(AnnoucementDetail {
      desc: text.to_string(),
      uri: link.to_owned(),
    });
  }
}
