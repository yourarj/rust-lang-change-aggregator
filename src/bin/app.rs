use std::error::Error;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
  rust_lang_change_aggregator::parse_root_doc().await?;
  Ok(())
}
