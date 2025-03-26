use change::Diff;
use aok::{OK, Result};

use crate::src::Src;

pub struct Save {
  pub diff_li: Vec<Diff>,
  pub src: Src,
}

impl Save {
  pub fn save(&self) -> Result<()> {
    for i in &self.diff_li {
      xerr::log!(i.save());
    }
    xerr::log!(self.src.save());
    OK
  }
}
