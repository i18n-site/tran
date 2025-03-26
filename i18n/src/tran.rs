use std::path::Path;

use zstd::bulk::compress;
use aok::{OK, Void};

use crate::{I18N_API, ScanResult, scan};

pub async fn tran(root: &Path) -> Void {
  use pb_jelly::Message;

  let ScanResult { tran, save, total } = scan(root)?;

  if tran.update_li.is_empty() && tran.tran_li.is_empty() {
    return OK;
  }

  let payload = compress(&tran.serialize_to_vec(), 9)?;

  let url = format!("{}tran", &*I18N_API);

  let key = ireq::req(ireq::REQ.put(url).body(payload)).await?;

  crate::recv(root, key, total).await?;

  save.save()?;

  OK
}
