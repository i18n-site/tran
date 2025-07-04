# tran_term

```rust
#![feature(let_chains)]

use std::collections::{HashMap, HashSet};

use daachorse::{CharwiseDoubleArrayAhoCorasick, CharwiseDoubleArrayAhoCorasickBuilder, MatchKind};
use unicode_categories::UnicodeCategories;
use unicode_segmentation::UnicodeSegmentation;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("daachorse: {0}")]
  Daachorse(daachorse::errors::DaachorseError),
}

fn capitalize_first_letter(s: impl AsRef<str>) -> String {
  let mut chars = s.as_ref().chars();
  match chars.next() {
    Some(first_char) => first_char.to_uppercase().to_string() + chars.as_str(),
    None => String::new(),
  }
}

pub struct Term {
  pub map: HashMap<String, String>,
  pub ac: CharwiseDoubleArrayAhoCorasick<usize>,
}

// 判断是否需要插入空格
pub fn word_push(li: &mut Vec<String>, txt: impl Into<String>) {
  let txt = txt.into();
  if let Some(last) = li.last()
    && let Some(last) = last.chars().last()
    && let Some(first) = txt.chars().next()
  {
    let t = last.to_string() + &first.to_string();
    if !t.contains('_') {
      let mut t = t.split_word_bounds();
      if let (Some(_), None) = (t.next(), t.next()) {
        li.push(" ".into())
      };
    }
  }
  li.push(txt);
}

impl Term {
  pub fn load<S1: AsRef<str>, S2: Into<String>>(
    from_iter: impl IntoIterator<Item = S1>,
    to_iter: impl IntoIterator<Item = S2>,
  ) -> Result<Self, Error> {
    let mut map = HashMap::new();

    let mut exist = HashSet::new();

    for (key, value) in from_iter.into_iter().zip(to_iter) {
      let lower_key = key.as_ref().to_lowercase();
      if lower_key.is_empty() || exist.contains(&lower_key) {
        continue;
      }
      exist.insert(lower_key.clone());

      let value = value.into();
      map.insert(lower_key, value);
    }

    match CharwiseDoubleArrayAhoCorasickBuilder::new()
      .match_kind(MatchKind::LeftmostLongest)
      .build(map.keys())
    {
      Ok(ac) => Ok(Term { map, ac }),
      Err(err) => Err(Error::Daachorse(err)),
    }
  }

  pub fn replace(&self, txt: impl AsRef<str>, fmt: impl Fn(String) -> String) -> Option<String> {
    let txt = txt.as_ref();
    let txt_lower = txt.to_lowercase();
    let mut li = vec![];
    let mut pos = 0;
    for mat in self.ac.leftmost_find_iter(&txt_lower) {
      let start = mat.start();
      let end = mat.end();
      let matched = &txt_lower[start..end];

      macro_rules! is_word {
        ($prev:expr, $next:expr) => {
          if let Some(last) = $prev.chars().last()
            && let Some(first) = $next.chars().next()
          {
            let t = last.to_string() + &first.to_string();
            if !t.contains('_') {
              let mut t = t.split_word_bounds();
              match (t.next(), t.next()) {
                (Some(_), None) => continue,
                _ => {}
              };
            }
          }
        };
      }

      if let Some(val) = self.map.get(matched) {
        let mut val = val.to_owned();

        is_word!(&txt[..start], matched);
        is_word!(matched, &txt[end..]);

        if start > pos {
          word_push(&mut li, &txt[pos..start]);
        }

        let org = &txt[start..end];
        if let Some(c) = org.chars().next()
          && (
            c.is_uppercase() || start == 0
            // 行首大写
          )
        {
          let pos = c.to_string().len();
          if org.len() > 1 && org[pos..].chars().all(char::is_uppercase) {
            val = val.to_uppercase();
          } else {
            val = capitalize_first_letter(val);
          }
        } else if let Some(last) = txt[..start].trim_end().chars().last() {
          if !"{}()_*[]~".contains(last) && last.is_punctuation() {
            val = capitalize_first_letter(val)
          }
        }

        word_push(&mut li, fmt(val));
      }
      pos = end;
    }
    if !li.is_empty() {
      if pos < txt.len() {
        word_push(&mut li, &txt[pos..]);
      }
      let r = li.concat();
      return Some(r);
    }
    None
  }
}
```

## About

This project is an open-source component of [i18n.site ⋅ Internationalization Solution](https://i18n.site).

* [i18 : MarkDown Command Line Translation Tool](https://i18n.site/i18)

  The translation perfectly maintains the Markdown format.

  It recognizes file changes and only translates the modified files.

  The translated Markdown content is editable; if you modify the original text and translate it again, manually edited translations will not be overwritten (as long as the original text has not been changed).

* [i18n.site : MarkDown Multi-language Static Site Generator](https://i18n.site/i18n.site)

  Optimized for a better reading experience

## 关于

本项目为 [i18n.site ⋅ 国际化解决方案](https://i18n.site) 的开源组件。

* [i18 :  MarkDown命令行翻译工具](https://i18n.site/i18)

  翻译能够完美保持 Markdown 的格式。能识别文件的修改，仅翻译有变动的文件。

  Markdown 翻译内容可编辑；如果你修改原文并再次机器翻译，手动修改过的翻译不会被覆盖（如果这段原文没有被修改）。

* [i18n.site : MarkDown多语言静态站点生成器](https://i18n.site/i18n.site) 为阅读体验而优化。
