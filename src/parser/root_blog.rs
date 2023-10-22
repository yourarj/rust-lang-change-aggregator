use std::error::Error;

use crate::dto::annoucement::RustVersionAnnoucement;

use super::Parse;

/// RUST Blog URL
pub const RUST_BLOG_URL: &str = "https://blog.rust-lang.org/";

pub async fn parse_root_doc() -> Result<(), Box<dyn Error>> {
  let client = reqwest::Client::new();

  let text_resp = client
    .get(RUST_BLOG_URL)
    .send()
    .await?
    .error_for_status()?
    .text()
    .await?;

  RustVersionAnnoucement::parse(&text_resp);
  Ok(())
}
