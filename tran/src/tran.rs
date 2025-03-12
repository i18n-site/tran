use std::collections::HashMap;

use tran_term::Term;
use aok::Result;
use xhash::xhash;
use kanal::Receiver;
use lang::Lang;

use crate::{Cache, Parse, Traner, tran};

pub struct Tran<C: Cache + Clone + Send + 'static> {
  pub term: HashMap<Lang, Term>,
  pub from: Lang,
  pub to_li: Vec<Lang>,
  pub cache: C,
}

impl<C: Cache + Clone + Send + 'static> Tran<C> {
  pub async fn tran<P: Parse, T: Traner>(
    &self,
    txt: impl Into<String>,
  ) -> Result<Receiver<(Lang, Result<Vec<String>>)>> {
    let from_lang = self.from;
    let txt = txt.into();
    let li: Vec<_> = P::parse(&txt)?
      .into_iter()
      .map(|r| txt[r.start..r.end].to_owned())
      .collect();

    let hash_li: Vec<_> = li.iter().map(|i| xhash(i.as_bytes())).collect();

    let (sender, recv) = kanal::unbounded();
    for to_lang in &self.to_li {
      let to_lang = *to_lang;
      if to_lang == self.from {
        continue;
      }

      let mut hash_li = hash_li.clone();

      let li = match self.term.get(&to_lang) {
        None => li.clone(),
        Some(term) => li
          .iter()
          .enumerate()
          .map(|(pos, i)| match term.replace(i) {
            Some(s) => {
              hash_li[pos] = xhash(s.as_bytes());
              s
            }
            None => i.to_owned(),
          })
          .collect(),
      };

      let sender = sender.clone();
      let cache = self.cache.clone();
      tokio::spawn(async move {
        let result = tran::<T>(from_lang, to_lang, cache, li, hash_li).await;
        xerr::log!(sender.send((to_lang, result)));
      });
    }
    Ok(recv)
  }
}
