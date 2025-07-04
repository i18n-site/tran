use std::{collections::HashSet, ops::Range};

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
  fn restore(self, src_li: &[String], traned_li: Vec<String>) -> Vec<String>;
  fn new(from_lang: u16, to_lang: u16, li: &[String]) -> (Self, Vec<String>)
  where
    Self: Sized;
}

pub struct ParseResult {
  pub range_li: Vec<Range<usize>>,
  pub title_pos: HashSet<usize>,
}

pub trait Parser {
  type Conv: Conv + Send;

  fn parse(txt: &str) -> Result<ParseResult>;
}
