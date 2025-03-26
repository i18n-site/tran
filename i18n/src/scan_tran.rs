use std::{
  collections::{HashMap, HashSet},
  fs,
  path::Path,
};

use txtfmt::txtfmt;
use lang::LANG;
use proto_tran::{LangTxt, RelTxt, UpdateLi};
use globset::GlobMatcher;
use aok::Result;

use crate::{filetype, src::Src, state, tran_yml::FromTo};

fn fmt_read(path: &Path) -> std::io::Result<String> {
  Ok(txtfmt(fs::read_to_string(path)?))
}

// pub fn scan_scan_tran(globset: &globset::GlobMatcher, ft: &FromTo, diff: &mut change::Diff) {}

pub fn scan_tran(
  globset: GlobMatcher,
  ft_li: Vec<FromTo>,
  src: &Src,
  ft_rel: &mut HashMap<FromTo, Vec<RelTxt>>,
  update_li: &mut Vec<UpdateLi>,
  term_changed_lang_set: &HashSet<u32>,
  get_state: &mut state::Get,
) {
  let root = get_state.root.clone();
  for ft in ft_li {
    // for tran
    let from_lang = ft.from_lang;
    let lang_dir = root.join(lang::CODE[from_lang as usize]);
    let read = |rel: &str| -> Result<RelTxt> {
      let fp = lang_dir.join(rel);
      Ok(RelTxt {
        rel: rel.to_owned(),
        txt: fmt_read(&fp)?,
      })
    };

    let mut exist = HashSet::new();

    get_state.get(from_lang, |state| {
      // 需要翻译的路径
      let tran_rel_li = ft_rel.entry(ft.clone()).or_default();
      let mut retain = |i: &String| {
        if globset.is_match(i) {
          exist.insert(i.to_owned());
          if let Ok(i) = xerr::ok!(read(i)) {
            tran_rel_li.push(i);
          }
          false
        } else {
          true
        }
      };
      // 如果术语表改变，全部重新翻译
      if term_changed_lang_set.contains(&from_lang) {
        for li in [&mut state.changed, &mut state.no_change] {
          li.retain(&mut retain);
        }
      } else {
        state.changed.retain(&mut retain);
        state.no_change.retain(|i| {
          if globset.is_match(i) {
            exist.insert(i.to_owned());
            false
          } else {
            true
          }
        });
      }
    });

    let mut to_update = HashMap::new();

    for to_lang in if ft.to_lang_li.is_empty() {
      LANG
    } else {
      &ft.to_lang_li
    } {
      let to_lang = *to_lang;
      let dir = root.join(lang::CODE[to_lang as usize]);
      get_state.get(to_lang, |state| {
        state.no_change.retain(|i| {
          if globset.is_match(i) {
            if !exist.contains(i) {
              xerr::log!(fs::remove_file(dir.join(i)));
            }
            false
          } else {
            true
          }
        });
        state.changed.retain(|i| {
          if globset.is_match(i) {
            let fp = dir.join(i);
            let filetype = filetype(i);
            if let Some(hash) = src.get(i, from_lang, to_lang) {
              if let Ok(txt) = fmt_read(&fp) {
                to_update
                  .entry((filetype, hash))
                  .or_insert_with(Vec::new)
                  .push(LangTxt { lang: to_lang, txt });
              }
            }
            if !exist.contains(i) {
              xerr::log!(fs::remove_file(fp));
            }
            false
          } else {
            true
          }
        });
      });
    }
    if !to_update.is_empty() {
      for ((filetype, hash), li) in to_update {
        update_li.push(UpdateLi {
          filetype,
          lang: from_lang,
          hash,
          li,
        })
      }
    }
  }
}
