use std::sync::Arc;

use titlecase::titlecase;
use remove_code_t::remove_code_t;
use logwait::logwait;
use aok::{OK, Result, Void};
use proto_tran::TranResult;
use tokio::task::JoinSet;
use tran_trait::Conv;
use kanal::AsyncSender as Sender;

use crate::{Parser, Src, Term, TranCache, TranParser, Traner};

pub struct Tran<C> {
  pub term: &'static Term,
  pub cache: C,
  pub sender: Sender<TranResult>,
  pub rel: String,
}

impl<C: TranCache> Tran<C> {
  pub fn li<P: TranParser>(
    self,
    src: Src<P>,
    ing: &mut JoinSet<()>,
    traner: impl Traner + Clone + Send + Sync + 'static,
    to_lang_li: Vec<u16>,
  ) {
    let mut updating = JoinSet::new();
    let from_lang = src.lang;
    ing.spawn(async move {
      drop::leak!(src);
      for to_lang in to_lang_li {
        let cache = self.cache.clone();
        let term = self.term.get(from_lang, to_lang);
        let sender = self.sender.clone();
        let traner = traner.clone();
        updating.spawn(tran(
          src,
          term,
          cache,
          to_lang,
          traner,
          sender,
          self.rel.clone(),
        ));
      }

      logwait(updating).await;
    });
  }
}

pub async fn tran<P: Parser>(
  src: &Src<P>,
  term: Arc<Option<tran_term::Term>>,
  cache: impl TranCache,
  to_lang: u16,
  traner: impl Traner,
  sender: Sender<TranResult>,
  rel: String,
) -> Void {
  let (code, msg) = match _tran(src, &term, cache, to_lang, traner).await {
    Ok(traned_li) => (0, range_merge::merge(&src.txt, &src.range_li, &traned_li)),
    Err(err) => {
      tracing::error!("{:?}", err);
      (1, err.to_string())
    }
  };

  sender
    .send(TranResult {
      id: 0,
      code,
      rel,
      lang: to_lang as _,
      msg,
    })
    .await?;
  OK
}

async fn _tran<P: Parser>(
  src: &Src<P>,
  term: &Option<tran_term::Term>,
  cache: impl TranCache,
  to_lang: u16,
  traner: impl Traner,
) -> Result<Vec<String>> {
  let cached = src.get(&cache, to_lang, term).await?;

  let li = if cached.to_tran_li.is_empty() {
    cached.cached
  } else {
    let to_tran_li = &cached.to_tran_li[..];
    let (conv, conved_li) = P::Conv::new(src.lang, to_lang, to_tran_li);

    let traned_li = traner.tran(src.lang, to_lang, &conved_li).await?;

    let traned_li = conv
      .restore(to_tran_li, traned_li)
      .into_iter()
      .map(remove_code_t)
      .collect::<Vec<_>>();

    cache
      .set(src.lang, to_lang, &cached.hash_li(), &traned_li)
      .await?;

    cached.merge(traned_li)
  };

  Ok(
    li.into_iter()
      .enumerate()
      .map(|(pos, i)| {
        if src.title_pos.contains(&pos) {
          titlecase(&i)
        } else {
          i
        }
      })
      .collect(),
  )
}
