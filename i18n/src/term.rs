use std::{fs, path::Path};

use saphyr::{
  Yaml,
  Yaml::{Hash, Null},
};
use aok::Result;
use proto_tran::{Dict, Term};

use crate::yml_lang;

pub fn dict_vec<'a>(
  dict: impl IntoIterator<Item = (&'a Yaml, &'a Yaml)>,
  from_word_li: &mut Vec<String>,
  to_word_li: &mut Vec<String>,
) {
  dict.into_iter().for_each(|(k, v)| {
    if let Some(k) = k.as_str()
      && let Some(v) = v.as_str()
      && !k.is_empty()
    {
      from_word_li.push(k.to_owned());
      to_word_li.push(v.to_owned());
    }
  })
}

pub fn term(root: &Path) -> Result<Vec<Term>> {
  let mut li = Vec::new();

  for entry in root.join(".i18n/term").read_dir()? {
    let entry = entry?;
    let path = entry.path();
    if let Some(from_lang) = yml_lang(&path) {
      let mut from_word_li = Vec::new();
      let mut to_word_li = Vec::new();
      let mut dict_li = Vec::new();

      for i in Yaml::load_from_str(&fs::read_to_string(&path)?)? {
        if let Hash(ref m) = i {
          for (k, v) in m {
            if *k == Null
              && let Hash(d) = v
            {
              dict_vec(d, &mut from_word_li, &mut to_word_li);
            } else if let Some(k) = k.as_str() {
              if let Some(to_lang) = lang::by_str(k)
                && let Hash(d) = v
              {
                let mut from_word_li = Vec::new();
                let mut to_word_li = Vec::new();

                dict_vec(d, &mut from_word_li, &mut to_word_li);

                if !from_word_li.is_empty() {
                  dict_li.push(Dict {
                    lang: to_lang as u32,
                    from_word_li,
                    to_word_li,
                  })
                }
              } else {
                eprint!("{:?} invalid key : {}", path, k)
              }
            }
          }
        }
      }
      if !from_word_li.is_empty() || !dict_li.is_empty() {
        li.push(Term {
          lang: from_lang,
          from_word_li,
          to_word_li,
          dict_li,
        })
      }
    } else {
      eprintln!("{:?} not valid term", path);
    }
  }

  Ok(li)
}
