use std::{collections::HashSet, fs, path::Path};

use saphyr::{
  Yaml,
  Yaml::{Hash, Null, String},
};
use globset::{Glob, GlobMatcher};
use aok::Result;

use crate::Error;

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Debug, Hash)]
pub struct FromTo {
  pub from_lang: u32,
  pub to_lang_li: Vec<u32>,
}

pub fn tran_yml(root: &Path) -> Result<Vec<(GlobMatcher, Vec<FromTo>)>> {
  let yml = root.join("tran.yml");
  if !yml.exists() {
    Err(Error::MissTranYml)?;
  }

  let mut path_map = vec![];

  for i in Yaml::load_from_str(&fs::read_to_string(yml)?)? {
    if let Hash(i) = i {
      for (path, map) in i {
        if let Hash(map) = map {
          if path == Null {
            path_map.push(("*".to_owned(), map));
          } else if let Some(path) = path.into_string() {
            path_map.push((path, map));
          }
        }
      }
    }
  }
  path_map.sort_by_key(|(p, _)| std::cmp::Reverse(p.len()));

  let mut glob_from_to = vec![];

  for (path, map) in path_map {
    match Glob::new(&path) {
      Ok(glob) => {
        let mut li = vec![];
        let glob = glob.compile_matcher();
        for (from_lang, to_lang_li) in map {
          if let String(from_lang) = from_lang {
            if let Some(from_lang) = lang::by_str(&from_lang) {
              let from_lang = from_lang as u32;
              if to_lang_li == Null {
                li.push(FromTo {
                  from_lang,
                  to_lang_li: Default::default(),
                })
              } else if let String(to_lang_li_str) = to_lang_li {
                let mut to_lang_li = HashSet::new();
                for i in to_lang_li_str.split_whitespace() {
                  if let Some(to_lang) = lang::by_str(i) {
                    to_lang_li.insert(to_lang as u32);
                  } else {
                    eprintln!(".118n/tran.yml error lang: {i}");
                  }
                }
                if !to_lang_li.is_empty() {
                  let mut to_lang_li: Vec<u32> = to_lang_li.into_iter().collect();
                  to_lang_li.sort();

                  li.push(FromTo {
                    from_lang,
                    to_lang_li,
                  });
                }
              }
            } else {
              eprintln!(".118n/tran.yml error lang: {from_lang}");
            }
          }
        }
        if !li.is_empty() {
          glob_from_to.push((glob, li));
        }
      }
      Err(e) => {
        eprintln!(".118n/tran.yml {path}: {e}");
        continue;
      }
    }
  }
  Ok(glob_from_to)
}

// pub struct TranSrc {
//   pub default_from: Lang,
//   pub rel_src: HashMap<String, Lang>,
// }
//
// impl TranSrc {
//   fn load(tran_yml: impl AsRef<Path>) -> Result<Self> {
//     let tran_yml = tran_yml.as_ref();
//     if !tran_yml.exists() {
//       Err(Error::MissTranYml)?;
//     }
//     let mut default_from = None;
//
//     let mut rel_src = HashMap::new();
//
//     for i in Yaml::load_from_str(&fs::read_to_string(tran_yml)?)? {
//       if let Hash(m) = i {
//         for (k, v) in m {
//           if let Some(v) = v.into_string() {
//             if let Some(p) = lang::CODE.index_of(&v.as_str()) {
//               let from_lang = lang::Lang::try_from(p as u16).unwrap();
//               if k == Null {
//                 default_from = Some(from_lang);
//               } else if let Some(rel) = k.into_string() {
//                 rel_src.insert(rel, from_lang);
//               }
//             } else {
//               eprintln!(".118n/tran.yml error lang: {v}");
//             }
//           }
//         }
//       }
//     }
//
//     match default_from {
//       None => Err(Error::MissDefaultFrom)?,
//       Some(default_from) => Ok(Self {
//         default_from,
//         rel_src,
//       }),
//     }
//   }
// }
