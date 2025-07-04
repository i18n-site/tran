#![feature(let_chains)]
#![feature(trait_alias)]
#![feature(new_range_api)]

use aok::Result;
use lang::LANG;
use proto_tran::{Filetype, RelTxt, TranResult};
use tokio::task::JoinSet;
use txtfmt::txtfmt;
use xhash::xhash;

mod cached;
pub(crate) use cached::Cached;
mod term;
pub use term::Term;
mod tran;
mod update;
pub use update::update;
pub mod src;
pub use src::Src;
use tran_trait::{Cache, Parser, Traner};

pub fn filetype(path: &str) -> Filetype {
  let len = path.len();
  if let Some(p) = path.rfind(".")
    && (p + 1) < len
  {
    return match &path[p + 1..] {
      "yml" => Filetype::Yml,
      _ => Filetype::Md,
    };
  }
  Filetype::Md
}

pub trait TranCache = Cache + Clone + Send + Sync + 'static;
pub trait TranParser = Parser + Send + Sync + 'static;

pub const LANG_MAX: u32 = LANG.len() as u32;

pub trait Tran {
  type Md: TranParser;
  type Yml: TranParser;

  #[allow(async_fn_in_trait)]
  async fn run(
    cache: impl TranCache,
    traner: impl Traner + Clone + Send + Sync + 'static,
    data: proto_tran::Tran,
  ) -> Result<kanal::AsyncReceiver<TranResult>> {
    let update_li = data
      .update_li
      .into_iter()
      .filter(|i| i.lang < LANG_MAX)
      .collect::<Vec<_>>();
    let tran_li = data
      .tran_li
      .into_iter()
      .filter(|i| i.from_lang < LANG_MAX)
      .collect::<Vec<_>>();

    drop::leak!(term = term::Term::new(data.term_li));

    // 用来区分内容类型是 Md 还是 Yaml，抽取需要翻译内容，返回类型是Src
    macro_rules! src {
      ($filetype:expr, $src:expr, $lang:expr, $body: expr) => {{
        src!(Md,Yml; $filetype, $src, $lang, $body)
      }};
      ($($ty:ident),+; $filetype:expr, $src:expr, $lang: expr, $body: expr) => {{
        use src::new;
        match $filetype {
          $(
            Filetype::$ty => {
              if let Ok(src) = xerr::ok!(new::<Self::$ty>($lang as u16, $src)){
                $body(src);
              }
            }
          ),+
        }
      }};
    }

    {
      // 获取待更新文件的源文本
      let src_li = {
        let lang_hash_li: Vec<_> = update_li
          .iter()
          .map(|i| ((i.lang as u16), &i.hash))
          .collect();
        cache.src_li(&lang_hash_li).await?
      };

      let mut ing = JoinSet::new();

      for (i, src) in update_li.into_iter().zip(src_li) {
        if let Some(src) = src {
          src!(i.filetype, src, i.lang, |src| {
            update::li(src, term, cache.clone(), &mut ing, i.li.clone());
          });
        }
      }

      while let Some(res) = ing.join_next().await {
        xerr::log!(res);
      }
    }

    let (sender, recv) = kanal::unbounded_async();

    if !tran_li.is_empty() {
      let mut ing = JoinSet::new();
      let mut to_set = Vec::with_capacity(tran_li.len());
      for i in tran_li {
        let from_lang = i.from_lang as u16;
        let to_lang_li = if i.to_lang_li.is_empty() {
          LANG
            .iter()
            .map(|i| *i as u16)
            .filter(|i| *i != from_lang)
            .collect::<Vec<_>>()
        } else {
          i.to_lang_li
            .into_iter()
            .filter(|i| *i < LANG_MAX)
            .map(|i| i as u16)
            .collect()
        };

        let mut to_set_li = Vec::with_capacity(i.li.len());

        for RelTxt { rel, txt } in &i.li {
          let txt = txtfmt(txt);
          let bin = txt.as_bytes();
          to_set_li.push((xhash(bin), bin.to_vec()));

          let filetype = filetype(rel);
          let sender = sender.clone();
          let cache = cache.clone();
          let traner = traner.clone();
          src!(filetype, txt, from_lang, |src| {
            tran::Tran {
              term,
              cache,
              sender,
              rel: rel.clone(),
            }
            .li(src, &mut ing, traner, to_lang_li.clone());
          })
        }

        to_set.push((from_lang, to_set_li));
      }

      ing.spawn(async move { xerr::log!(cache.set_src_li(to_set).await) });

      while let Some(res) = ing.join_next().await {
        xerr::log!(res);
      }
    }

    Ok(recv)
  }
}
