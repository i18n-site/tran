use std::{io::Cursor, path::Path};

use futures_util::{SinkExt, TryStreamExt};
use aok::{OK, Void};
use bytes::Bytes;
use pbar::pbar;
use reqwest_websocket::{Message, RequestBuilderExt};
use zstd::stream::decode_all;

use crate::I18N_API;

pub async fn recv(root: &Path, key: Bytes, total: u64) -> Void {
  let url = format!(
    "{}tran-{}",
    if let Some(remain) = I18N_API.strip_prefix("https://") {
      format!("wss://{remain}")
    } else {
      format!("ws://{}", &I18N_API[7..])
    },
    ub64::b64e(&key)
  );
  let mut bar = pbar(total);
  let mut oked = 0;
  'out: loop {
    let response = reqwest::Client::default()
      .get(&url)
      .upgrade()
      .send()
      .await?;
    let mut websocket = response.into_websocket().await?;
    while let Ok(Some(message)) = websocket.try_next().await {
      if let Message::Binary(bin) = message {
        if bin.is_empty() {
          websocket.send(Message::Binary(b"".into())).await?;
          websocket
            .close(reqwest_websocket::CloseCode::Normal, None)
            .await?;
          break 'out;
        } else {
          let cursor: Cursor<Vec<u8>> = Cursor::new(bin);
          let bin = decode_all(cursor)?;
          if let Ok::<proto_tran::TranResult, _>(result) =
            xerr::ok!(pb_jelly::Message::deserialize_from_slice(&bin))
          {
            let lang = lang::CODE[result.lang as usize];
            bar.inc(1);
            if result.code == 0 {
              oked += 1;
              xerr::log!(ifs::wstr(root.join(lang).join(&result.rel), result.msg));
              bar.set_message(format!("✅ {} → {lang}", result.rel));
            } else {
              bar.println(format!("❌ {} → {lang} : {}", result.rel, result.msg));
            }
            websocket
              .send(Message::Binary(intbin::to_bin(result.id).into()))
              .await?;
          }
        }
      }
    }
  }
  bar.finish_with_message(format!("✅ {oked} / {total}"));
  OK
}
