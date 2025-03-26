use std::{collections::HashMap, path::PathBuf};

use index_of::IndexOf;
use change::Diff;

use crate::EXT_LI;

pub struct State {
  pub changed: Vec<String>,
  pub no_change: Vec<String>,
}

pub struct Get {
  pub lang_diff: HashMap<u32, State>,
  pub dir_hash: PathBuf,
  pub diff_li: Vec<Diff>,
  pub root: PathBuf,
}

impl Get {
  pub fn get(&mut self, lang: u32, mut run: impl FnMut(&mut State)) {
    if let Some(state) = self.lang_diff.get_mut(&lang) {
      run(state);
    } else if let Ok(scan) = xerr::ok!(change::Scan::new(
      self.root.join(lang::CODE[lang as usize]),
      |build| {
        build.filter_entry(|entry| {
          if entry.path().is_dir() {
            return true;
          }
          if let Some(ext) = entry.path().extension() {
            let ext = ext.to_string_lossy();
            return EXT_LI.index_of(&ext.as_str()).is_some();
          }
          false
        })
      }
    )) && let Ok(diff) = xerr::ok!(
      scan.diff(
        self
          .dir_hash
          .join(format!("{}.yml", lang::CODE[lang as usize]))
      )
    ) {
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
      self.diff_li.push(diff);
      self.lang_diff.insert(lang, state);
    }
  }
}
