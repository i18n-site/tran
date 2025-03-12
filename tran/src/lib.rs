use std::ops::Range;

use s_::EMPTY;
use lang::Lang;
use aok::{Result, Void};

mod tran;
pub use tran::Tran;
mod update;
pub use update::update;

pub trait Traner {
  fn tran(
    from_lang: Lang,
    to_lang: Lang,
    txt_li: &[String],
  ) -> impl std::future::Future<Output = Result<Vec<Option<String>>>> + Send;
}

pub trait Parse {
  fn parse(txt: &str) -> Result<Vec<Range<usize>>>;
}

pub trait Cache {
  fn get(
    &self,
    from_lang: Lang,
    to_lang: Lang,
    hash_li: &[Vec<u8>],
  ) -> impl std::future::Future<Output = Result<Vec<Option<String>>>> + Send;

  fn set(
    &self,
    from_lang: Lang,
    to_lang: Lang,
    hash_li: &[Vec<u8>],
    li: &Vec<String>,
  ) -> impl std::future::Future<Output = Void> + Send;

  fn set_user(
    &self,
    from_lang: Lang,
    to_lang: Lang,
    hash_str_li: &[(Vec<u8>, String)],
  ) -> impl std::future::Future<Output = Void> + Send;
}

async fn tran<T: Traner>(
  from_lang: Lang,
  to_lang: Lang,
  cache: impl Cache,
  li: Vec<String>,
  hash_li: Vec<Vec<u8>>,
) -> Result<Vec<String>> {
  let mut result = Vec::with_capacity(li.len());
  let mut to_tran = vec![];
  if !li.is_empty() {
    // todo 术语替换并计算有区别句子的hash

    let j2f = from_lang == Lang::Zh && to_lang == Lang::ZhTw;
    let f2j = from_lang == Lang::ZhTw && to_lang == Lang::Zh;

    let mut pos_li = Vec::new();
    for (pos, (s, c)) in li
      .into_iter()
      .zip(cache.get(from_lang, to_lang, &hash_li).await?.into_iter())
      .enumerate()
    {
      if let Some(c) = c {
        result.push(c);
      } else if j2f {
        result.push(cnu::j2f(s));
      } else if f2j {
        result.push(cnu::f2j(s));
      } else {
        to_tran.push(s);
        result.push(EMPTY);
        pos_li.push(pos);
      }
    }
    if !to_tran.is_empty() {
      let to_li = T::tran(from_lang, to_lang, &to_tran[..]).await?;
      for (pos, to) in pos_li.into_iter().zip(to_li) {
        if let Some(to) = to {
          result[pos] = to;
        }
      }
      cache.set(from_lang, to_lang, &hash_li, &result).await?;
    }
  }

  Ok(result)
}
