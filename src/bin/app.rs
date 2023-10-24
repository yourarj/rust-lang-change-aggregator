use std::error::Error;

use rust_lang_change_aggregator::{
  parse_announcement_detail, parse_root_doc, RUST_BLOG_URL,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
  let parse_result = parse_announcement_detail(parse_root_doc().await?).await?;

  for (annc, detail) in parse_result {
    let seq = annc.seq();
    let version = annc.version();
    let uri = annc.uri();
    println!("## [{seq:03}: {version}]({RUST_BLOG_URL}{uri})",);
    for det in detail {
      let desc = det.desc();
      let anchor = det.uri();
      println!("- ### [{desc}]({RUST_BLOG_URL}{uri}{anchor})");
    }
  }
  Ok(())
}
