use std::{
  collections::{HashMap, HashSet},
  marker::PhantomData,
  ops::Range,
};

use aok::Result;
use tran_trait::ParseResult;
use xhash::xhash;

use crate::{Cached, Parser, TranCache};

pub struct Src<P: Parser> {
  pub lang: u16,
  pub hash_li: Vec<Vec<u8>>,
  pub range_li: Vec<Range<usize>>,
  pub title_pos: HashSet<usize>,
  pub txt: String,
  _parse: PhantomData<P>,
}

pub fn new<P: crate::Parser>(lang: u16, txt: impl Into<String>) -> Result<Src<P>> {
  let txt = txt.into();
  let ParseResult {
    range_li,
    title_pos,
  } = P::parse(&txt)?;
  Ok(Src {
    lang,
    hash_li: range_li
      .iter()
      .map(|i| xhash(&txt[i.start..i.end]))
      .collect(),
    title_pos,
    range_li,
    txt,
    _parse: PhantomData,
  })
}

pub struct Cache {
  pub cached: Vec<Option<String>>,
  pub hash_li: Vec<Vec<u8>>,
  pub pos_term: HashMap<usize, String>,
}

impl<P: Parser> Src<P> {
  pub async fn cache(
    &self,
    cache: &impl TranCache,
    to_lang: u16,
    term: &Option<tran_term::Term>,
  ) -> Result<Cache> {
    // TODO optimize for zh - zh-tw
    let mut pos_term = HashMap::new();
    let txt_li: Vec<&str> = self
      .range_li
      .iter()
      .map(|i| &self.txt[i.start..i.end])
      .collect();

    macro_rules! cache {
      ($hash_li:expr) => {
        cache.get(self.lang, to_lang, $hash_li, &txt_li[..]).await?
      };
    }

    // 如果有术语，先抽取，然后翻译完成后再还原
    Ok(if let Some(term) = term {
      let mut hash_li = self.hash_li.clone();
      for (pos, t) in txt_li.iter().enumerate() {
        if let Some(t) = term.replace(t, |s| {
          let s = htmlize::escape_text(s);
          format!(r#"<code t>{s}</code>"#)
        }) {
          hash_li[pos] = xhash(&t);
          pos_term.insert(pos, t);
        }
      }
      Cache {
        cached: cache!(&hash_li),
        hash_li,
        pos_term,
      }
    } else {
      Cache {
        cached: cache!(&self.hash_li),
        hash_li: self.hash_li.clone(),
        pos_term,
      }
    })
  }

  pub async fn get(
    &self,
    cache: &impl TranCache,
    to_lang: u16,
    term: &Option<tran_term::Term>,
  ) -> Result<Cached> {
    let Cache {
      cached,
      mut pos_term,
      ..
    } = self.cache(cache, to_lang, term).await?;
    let mut to_tran_li: Vec<String> = vec![];
    let mut to_tran_pos: Vec<usize> = vec![];
    Ok(Cached {
      cached: cached
        .into_iter()
        .enumerate()
        .map(|(pos, s)| {
          if let Some(s) = s {
            s
          } else {
            let range = &self.range_li[pos];
            to_tran_li.push(
              pos_term
                .remove(&pos)
                .unwrap_or(self.txt[range.start..range.end].to_owned()),
            );
            to_tran_pos.push(pos);

            s_::EMPTY
          }
        })
        .collect(),
      to_tran_li,
      to_tran_pos,
    })
  }
}
