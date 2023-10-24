use std::error::Error;

use crate::dto::{
  announcement::RustVersionAnnouncement, announcement_detail::AnnoucementDetail,
};
use std::collections::BTreeMap;

use super::{root_blog::RUST_BLOG_URL, Parse};

pub async fn parse_announcement_detail(
  announcements: Vec<RustVersionAnnouncement>,
) -> Result<
  BTreeMap<RustVersionAnnouncement, Vec<AnnoucementDetail>>,
  Box<dyn Error>,
> {
  let client = reqwest::Client::new();
  let mut map = BTreeMap::new();

  for ann in announcements {
    let text_resp = client
      .get(format!("{}{}", RUST_BLOG_URL, ann.uri()))
      .send()
      .await?
      .error_for_status()?
      .text()
      .await?;

    map.insert(ann, AnnoucementDetail::parse(&text_resp));
  }

  Ok(map)
}
