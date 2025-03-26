use std::sync::Arc;

use dashmap::DashMap;

pub type MapLangFromLiToLi = DashMap<u16, (Vec<String>, Vec<String>)>;

pub struct Term {
  pub map: DashMap<(u16, u16), Arc<Option<tran_term::Term>>>,
  pub term_li: DashMap<u16, (Vec<String>, Vec<String>, MapLangFromLiToLi)>,
}

impl Term {
  pub fn new(term_li: Vec<proto_tran::Term>) -> Self {
    Term {
      map: DashMap::new(),
      term_li: term_li
        .into_iter()
        .map(|term| {
          (
            term.lang as u16,
            (
              term.from_word_li,
              term.to_word_li,
              term
                .dict_li
                .into_iter()
                .map(|dict| (dict.lang as u16, (dict.from_word_li, dict.to_word_li)))
                .collect(),
            ),
          )
        })
        .collect(),
    }
  }

  pub fn get(&self, from_lang: u16, to_lang: u16) -> Arc<Option<tran_term::Term>> {
    self
      .map
      .entry((from_lang, to_lang))
      .or_insert_with(|| {
        if let Some(t) = self.term_li.get(&from_lang) {
          let (default_fli, default_tli, m) = t.value();
          let default_fli = &default_fli[..];
          let default_tli = &default_tli[..];

          if let Some((_, (fli, tli))) = m.remove(&to_lang) {
            if let Ok(r) = xerr::ok!(tran_term::Term::load(
              fli.iter().chain(default_fli),
              tli.iter().chain(default_tli)
            )) {
              return Some(r).into();
            }
          } else if let Ok(r) = xerr::ok!(tran_term::Term::load(default_fli, default_tli)) {
            return Some(r).into();
          }
        }

        None.into()
      })
      .value()
      .clone()
  }
}
