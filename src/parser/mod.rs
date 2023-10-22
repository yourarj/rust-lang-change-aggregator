//! Parser for Blogs

mod root_blog;
pub use root_blog::parse_root_doc;

/// Parse trait for parsing string
pub trait Parse<T> {
  fn parse(input: &str) -> Vec<T>;
}
