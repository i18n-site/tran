#![feature(doc_auto_cfg)]
#![feature(doc_cfg)]

pub trait Traner {
  fn parse(&mut self) -> Result<String>;
  fn restore(&mut self) -> Result<String>;
}
