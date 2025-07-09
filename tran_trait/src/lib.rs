#![feature(doc_auto_cfg)]
#![feature(doc_cfg)]

use aok::Result;

pub trait FileTypeTran {
  fn parse(&mut self) -> Result<Vec<String>>;
  fn restore<S: AsRef<S>>(&mut self, li: impl IntoIterator<Item = S>) -> Result<String>;
}
