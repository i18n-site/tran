use std::sync::Arc;

use xhash::xhash;
use aok::{OK, Void};
use txtfmt::txtfmt;
use tokio::task::JoinSet;
use logwait::logwait;
use proto_tran::LangTxt;

use crate::{LANG_MAX, Parser, Src, Term, TranCache, TranParser};

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("line number same")]
  LineNumberSame,
}

pub fn li<P: TranParser>(
  src: Src<P>,
  term: &'static Term,
  cache: impl TranCache,
  ing: &mut JoinSet<()>,
  li: Vec<LangTxt>,
) {
  let mut updating = JoinSet::new();

  let from_lang = src.lang;
  ing.spawn(async move {
    drop::leak!(src);
    for LangTxt { lang, txt } in li {
      if lang < LANG_MAX {
        let cache = cache.clone();
        let to_lang = lang as u16;
        let txt = txtfmt(txt);
        let term = term.get(from_lang, to_lang);
        updating.spawn(update(src, term, cache, to_lang, txt));
      }
    }
    logwait(updating).await;
  });
}

pub async fn update<P: Parser>(
  src: &Src<P>,
  term: Arc<Option<tran_term::Term>>,
  cache: impl TranCache,
  to_lang: u16,
  to_txt: String,
) -> Void {
  let li = src.get(&cache, to_lang, &term).await?;

  let mut to_set = vec![];
  if let Ok(to_li) = xerr::ok!(P::parse(&to_txt)) {
    if to_li.len() != li.len() {
      return Err(Error::LineNumberSame.into());
    }

    to_li.into_iter().zip(li).for_each(|(range, src)| {
      if let Some(src) = src {
        let to = &to_txt[range.start..range.end];
        if src != to {
          to_set.push((xhash(src), to));
        }
      }
    });
  }

  if to_set.is_empty() {
    return OK;
  }
  cache.set_user(src.lang, to_lang, to_set).await
}
