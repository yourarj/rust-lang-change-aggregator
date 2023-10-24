//! Parser for Blogs

mod root_blog;
pub use root_blog::parse_root_doc;
pub use root_blog::RUST_BLOG_URL;
mod announcement_detail;
pub use announcement_detail::parse_announcement_detail;

/// Parse trait for parsing string
pub trait Parse<T> {
  fn parse(input: &str) -> Vec<T>;
}
