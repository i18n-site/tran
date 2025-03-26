use std::{
  collections::HashSet,
  path::{Path, PathBuf},
};

use change::Diff;
use aok::Result;

use crate::yml_lang;

pub fn term_changed(root: &Path) -> Result<(HashSet<u32>, Diff)> {
  let mut set = HashSet::new();
  let scan = change::Scan::new(root.join("term"), |build| build)?;
  let diff = scan.diff(root.join("hash/term.yml"))?;

  for (path, _) in &diff.changed {
    let path: PathBuf = path.into();
    if let Some(lang) = yml_lang(&path) {
      set.insert(lang);
    }
  }

  Ok((set, diff))
}
