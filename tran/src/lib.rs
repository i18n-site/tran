use aok::{OK, Void};
use tran_api::Job;

pub trait Traner {
  fn parse(&mut self) -> Result<String>;
  fn restore(&mut self) -> Result<String>;
}

pub fn tran(job: Job) -> Void {
  /*
  缓存中保存
  */
  for i in job.update_li {}

  for i in job.tran_li {}
  OK
}
