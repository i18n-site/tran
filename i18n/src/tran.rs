use std::{
  collections::{HashMap, HashSet},
  fs,
  path::Path,
};

use globset::GlobMatcher;
use index_of::IndexOf;
use change::Diff;
use aok::{OK, Result, Void};
use proto_tran::RelTxt;

use crate::{term, term_changed, tran_yml::FromTo};

pub struct Save {
  li: Vec<Diff>,
}

impl Save {
  pub fn save(&self) -> Result<()> {
    for i in &self.li {
      i.save()?;
    }
    OK
  }
}

// pub fn scan_scan_lang(globset: &globset::GlobMatcher, ft: &FromTo, diff: &mut change::Diff) {}

pub(crate) struct State {
  changed: Vec<String>,
  no_change: Vec<String>,
}

fn scan_lang<const N: usize>(
  globset: GlobMatcher,
  ft_li: Vec<FromTo>,
  ext_li: &'static [&str; N],
  root: &Path,
  dir_hash: &Path,
  ft_rel: &mut HashMap<FromTo, Vec<RelTxt>>,
  lang_diff: &mut HashMap<u32, State>,
  diff_li: &mut Vec<Diff>,
  term_changed_lang_set: &HashSet<u32>,
) {
  for ft in ft_li {
    // for tran
    let lang = ft.from_lang;
    let lang_dir = root.join(lang::CODE[lang as usize]);
    let read = |rel: &str| -> Result<RelTxt> {
      let fp = lang_dir.join(rel);
      Ok(RelTxt {
        rel: rel.to_owned(),
        txt: fs::read_to_string(&fp)?,
      })
    };

    let run = |state: &mut State| {
      let ft_rel = ft_rel.entry(ft).or_insert_with(Vec::new);
      let mut retain = |i: &String| {
        if globset.is_match(i) {
          if let Ok(i) = xerr::ok!(read(i)) {
            ft_rel.push(i);
          }
          false
        } else {
          true
        }
      };
      if term_changed_lang_set.contains(&lang) {
        for li in [&mut state.changed, &mut state.no_change] {
          li.retain(&mut retain);
        }
      } else {
        state.changed.retain(&mut retain);
      }
    };

    if let Some(state) = lang_diff.get_mut(&lang) {
      run(state);
    } else {
      if let Ok(scan) = xerr::ok!(change::Scan::new(
        root.join(lang::CODE[lang as usize]),
        |build| {
          build.filter_entry(|entry| {
            if entry.path().is_dir() {
              return true;
            }
            if let Some(ext) = entry.path().extension() {
              let ext = ext.to_string_lossy();
              return ext_li.index_of(&ext.as_str()).is_some();
            }
            false
          })
        }
      )) && let Ok(diff) = xerr::ok!(scan.diff(dir_hash.join(format!("{}.yml", lang))))
      {
        let mut state = State {
          changed: diff
            .changed
            .iter()
            .map(|(path, _)| path.to_owned())
            .collect(),
          no_change: diff
            .no_change
            .iter()
            .map(|(path, _)| path.to_owned())
            .collect(),
        };
        run(&mut state);
        diff_li.push(diff);
        lang_diff.insert(lang, state);
      }
    }
  }
}

pub fn scan<const N: usize>(
  root: &Path,
  ext_li: &'static [&str; N],
) -> Result<(proto_tran::Tran, Save)> {
  let dir_i18n = root.join(".i18n");

  let tran_yml = crate::tran_yml(&dir_i18n)?;
  let (term_changed_lang_set, term_diff) = term_changed(&dir_i18n)?;

  let mut diff_li = vec![term_diff];

  let dir_hash = dir_i18n.join("hash");
  let mut lang_diff = HashMap::new();
  let mut ft_rel = HashMap::new();

  for (globset, ft_li) in tran_yml {
    scan_lang(
      globset,
      ft_li,
      ext_li,
      &root,
      &dir_hash,
      &mut ft_rel,
      &mut lang_diff,
      &mut diff_li,
      &term_changed_lang_set,
    );
  }

  let update_li = Default::default();
  let mut tran_li = Vec::with_capacity(ft_rel.len());

  for (i, li) in ft_rel {
    if !li.is_empty() {
      tran_li.push(proto_tran::TranLi {
        from_lang: i.from_lang,
        to_lang_li: i.to_lang_li,
        li,
      });
    }
  }

  Ok((
    proto_tran::Tran {
      update_li,
      tran_li,
      term_li: term(root)?,
    },
    Save { li: diff_li },
  ))
}

pub async fn tran(root: &Path) -> Void {
  let (payload, save) = scan(root, &["yml", "md"])?;
  // dbg!(payload);
  save.save()?;
  OK
}
