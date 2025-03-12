use std::ops::Range;

use xhash::xhash;
use lang::Lang;
use tran_term::Term;
use aok::{OK, Void};

use crate::{Cache, Parse};

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("line number same")]
  LineNumberSame,
}

pub async fn update<P: Parse>(
  from_lang: Lang,
  to_lang: Lang,
  cache: &impl Cache,
  src: impl AsRef<str>,
  range_li: Vec<Range<usize>>,
  mut hash_li: Vec<Vec<u8>>,
  term: &Option<Term>,
  to: impl AsRef<str>,
) -> Void {
  let src = src.as_ref();
  let to = to.as_ref();
  let to_range_li = P::parse(to)?.into_iter();
  if to_range_li.len() != range_li.len() {
    return Err(Error::LineNumberSame.into());
  }
  if let Some(term) = term {
    for (pos, i) in range_li.into_iter().enumerate() {
      if let Some(r) = term.replace(&src[i.start..i.end]) {
        hash_li[pos] = xhash(r.as_bytes());
      }
    }
  }

  let exist = cache.get(from_lang, to_lang, &hash_li).await?;

  let mut to_set = vec![];
  for ((exist, hash), r) in exist.into_iter().zip(hash_li).zip(to_range_li) {
    if let Some(exist) = exist {
      let new_tran = src[r.start..r.end].to_owned();
      if exist != to[r.start..r.end] {
        to_set.push((hash, new_tran));
      }
    }
  }

  cache.set_user(from_lang, to_lang, &to_set).await?;
  OK
}
