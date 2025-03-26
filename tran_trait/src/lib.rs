use std::ops::Range;

use aok::Result;

mod cache;
pub use cache::Cache;

pub trait Traner {
  fn tran<S: AsRef<str> + Sync>(
    &self,
    from_lang: u16,
    to_lang: u16,
    txt_li: &[S],
  ) -> impl std::future::Future<Output = Result<Vec<String>>> + Send;
}

pub trait Conv {
  fn restore(self, txt: &str, range_li: Vec<Range<usize>>, traned_li: Vec<String>) -> Vec<String>;
  fn new(from_lang: u16, to_lang: u16, txt: &str, range_li: &[Range<usize>]) -> (Self, Vec<String>)
  where
    Self: Sized;
}

pub trait Parser {
  type Conv: Conv + Send;

  fn parse(txt: &str) -> Result<Vec<Range<usize>>>;
}
