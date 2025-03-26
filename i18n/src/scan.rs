use std::{collections::HashMap, path::PathBuf};

use lang::LANG;
use aok::Result;
use xhash::xhash;

use crate::{Save, scan_tran, state, term, term_changed};

pub struct ScanResult {
  pub tran: proto_tran::Tran,
  pub save: Save,
  pub total: u64,
}

pub fn scan(root: impl Into<PathBuf>) -> Result<ScanResult> {
  let root = root.into();
  let dir_i18n = root.join(".i18n");

  let tran_yml = crate::tran_yml(&dir_i18n)?;
  let (term_changed_lang_set, term_diff) = term_changed(&dir_i18n)?;

  let mut ft_rel = HashMap::new();

  let mut get_state = state::Get {
    diff_li: vec![term_diff],
    lang_diff: HashMap::new(),
    dir_hash: dir_i18n.join("hash").join("lang"),
    root,
  };

  let mut update_li = vec![];
  let mut src = crate::src::load(dir_i18n.join("src.yml"))?;

  for (globset, ft_li) in tran_yml {
    scan_tran(
      globset,
      ft_li,
      &src,
      &mut ft_rel,
      &mut update_li,
      &term_changed_lang_set,
      &mut get_state,
    );
  }

  let mut tran_li = Vec::with_capacity(ft_rel.len());

  let mut total = 0;

  for (i, li) in ft_rel {
    if !li.is_empty() {
      let to_lang_li = if i.to_lang_li.is_empty() {
        LANG
      } else {
        &i.to_lang_li[..]
      };

      for j in &li {
        let hash = xhash(&j.txt);
        for to in to_lang_li {
          src.add(&j.rel, i.from_lang, hash.clone(), *to);
        }
      }

      total += (li.len() * to_lang_li.len()) as u64;
      tran_li.push(proto_tran::TranLi {
        from_lang: i.from_lang,
        to_lang_li: i.to_lang_li,
        li,
      });
    }
  }

  Ok(ScanResult {
    tran: proto_tran::Tran {
      update_li,
      tran_li,
      term_li: term(&get_state.root)?,
    },
    save: Save {
      src,
      diff_li: get_state.diff_li,
    },
    total,
  })
}
