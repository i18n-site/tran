use std::{marker::PhantomData, ops::Range};

use aok::Result;
use lang::Lang;
use xhash::xhash;

use crate::{Parser, TranCache};

pub struct Src<P: Parser> {
  pub lang: u16,
  pub hash_li: Vec<Vec<u8>>,
  pub range_li: Vec<Range<usize>>,
  pub txt: String,
  _parse: PhantomData<P>,
}

pub fn new<P: crate::Parser>(lang: u16, txt: impl Into<String>) -> Result<Src<P>> {
  let txt = txt.into();
  let range_li = P::parse(&txt)?;
  Ok(Src {
    lang,
    hash_li: range_li
      .iter()
      .map(|i| xhash(&txt[i.start..i.end]))
      .collect(),
    range_li,
    txt,
    _parse: PhantomData,
  })
}

impl<P: Parser> Src<P> {
  fn conv(
    &self,
    conv: impl Fn(&str) -> String,
    term: &Option<tran_term::Term>,
  ) -> Vec<Option<String>> {
    if let Some(term) = term {
      let mut li = Vec::with_capacity(self.range_li.len());
      for i in &self.range_li {
        let t = &self.txt[i.start..i.end];
        match term.replace(t) {
          Some(t) => li.push(Some(conv(&t))),
          None => li.push(Some(conv(t))),
        }
      }
      return li;
    }
    self
      .range_li
      .iter()
      .map(|i| Some(conv(&self.txt[i.start..i.end])))
      .collect()
  }

  pub async fn get(
    &self,
    cache: &impl TranCache,
    to_lang: u16,
    term: &Option<tran_term::Term>,
  ) -> Result<Vec<Option<String>>> {
    if self.lang == (Lang::Zh as u16) && to_lang == (Lang::ZhTw as u16) {
      return Ok(self.conv(|s| cnu::j2f(s), term));
    }
    if self.lang == (Lang::ZhTw as u16) && to_lang == (Lang::Zh as u16) {
      return Ok(self.conv(|s| cnu::f2j(s), term));
    }
    if let Some(term) = term {
      let mut hash_li = self.hash_li.clone();
      for (pos, i) in self.range_li.iter().enumerate() {
        let t = &self.txt[i.start..i.end];
        if let Some(t) = term.replace(t) {
          hash_li[pos] = xhash(&t);
        }
      }
      return cache.get(self.lang, to_lang, &hash_li).await;
    }
    return cache.get(self.lang, to_lang, &self.hash_li).await;
  }
}
