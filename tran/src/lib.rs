use aok::{OK, Result, Void};
use tran_api::Job;
use tran_trait::FileTypeTran;

pub fn tran<F: FileTypeTran, const N: usize>(job: Job, file_type_tran: [F; N]) -> Void {
  /*
  缓存中保存
  */
  for i in job.update_li {}

  for i in job.tran_li {}
  OK
}
