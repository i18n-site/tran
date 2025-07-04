use xhash::xhash;

#[derive(Default)]
pub struct Cached {
  pub cached: Vec<String>,
  pub to_tran_li: Vec<String>,
  pub to_tran_pos: Vec<usize>,
}

impl Cached {
  pub fn merge(mut self, traned_li: Vec<String>) -> Vec<String> {
    for (pos, traned) in self.to_tran_pos.into_iter().zip(traned_li.into_iter()) {
      self.cached[pos] = traned;
    }
    self.cached
  }

  pub fn hash_li(&self) -> Vec<Vec<u8>> {
    self
      .to_tran_li
      .iter()
      .map(|s| xhash(s.as_bytes()))
      .collect()
  }
}
