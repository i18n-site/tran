use std::future::Future;

use aok::{Result, Void};
use lang::Lang;
use zhconv::{Variant, zhconv};

pub type HashTxt = (Vec<u8>, Vec<u8>);

pub trait Cache {
  fn get(
    &self,
    from_lang: u16,
    to_lang: u16,
    hash_li: &[Vec<u8>],
    txt_li: &[&str],
  ) -> impl Future<Output = Result<Vec<Option<String>>>> + Send
  where
    Self: Sync,
  {
    async move {
      if hash_li.is_empty() {
        return Ok(vec![]);
      }
      let ft = vb::e([from_lang as u64, to_lang as u64]);
      let mut r = self.user_get(&ft, hash_li).await?;
      let mut to_fetch = vec![];
      let mut pos_li = vec![];

      for (pos, (cached, hash)) in r.iter().zip(hash_li).enumerate() {
        if cached.is_none() {
          to_fetch.push(hash.clone());
          pos_li.push(pos);
        }
      }

      if !to_fetch.is_empty() {
        if to_lang == (Lang::Zh as u16) && from_lang == (Lang::ZhTw as u16) {
          for pos in pos_li {
            r[pos] = Some(zhconv(txt_li[pos], Variant::ZhCN))
          }
        } else if from_lang == (Lang::Zh as u16) && to_lang == (Lang::ZhTw as u16) {
          for pos in pos_li {
            r[pos] = Some(zhconv(txt_li[pos], Variant::ZhTW))
          }
        } else {
          for (pos, i) in pos_li
            .into_iter()
            .zip(self.global_get(&ft, &to_fetch[..]).await?.into_iter())
          {
            r[pos] = i;
          }
        }
      }

      Ok(r)
    }
  }

  fn global_get(
    &self,
    from_to: &[u8],
    hash_li: &[Vec<u8>],
  ) -> impl Future<Output = Result<Vec<Option<String>>>> + Send;

  fn user_get(
    &self,
    from_to: &[u8],
    hash_li: &[Vec<u8>],
  ) -> impl Future<Output = Result<Vec<Option<String>>>> + Send;

  fn set(
    &self,
    from_lang: u16,
    to_lang: u16,
    hash_li: &[Vec<u8>],
    li: &[String],
  ) -> impl Future<Output = Void> + Send;

  fn set_src_li(
    &self,
    li: Vec<(
      // lang
      u16,
      // hash txt
      Vec<HashTxt>,
    )>,
  ) -> impl Future<Output = Void> + Send;

  fn user_set(
    &self,
    from_lang: u16,
    to_lang: u16,
    hash_str_li: Vec<(Vec<u8>, &str)>,
  ) -> impl Future<Output = Void> + Send;

  fn src_li(
    &self,
    hash_li: &[(u16, &Vec<u8>)],
  ) -> impl Future<Output = Result<Vec<Option<String>>>> + Send;
}
