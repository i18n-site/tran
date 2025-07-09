#![feature(doc_auto_cfg)]
#![feature(doc_cfg)]

use aok::Result;

pub trait FileTypeTran {
  fn parse(&mut self) -> Result<String>;
  fn restore(&mut self) -> Result<String>;
}
