#![feature(str_as_str)]
#![feature(str_split_remainder)]
#![feature(let_chains)]

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("miss .i18n/tran.yml")]
  MissTranYml,

  #[error(".i18n/tran.yml miss default from lang")]
  MissDefaultFrom,
}

mod term;
pub use term::term;

mod tran;
pub use tran::tran;

mod yml_lang;
pub use yml_lang::yml_lang;

mod tran_yml;
pub use tran_yml::tran_yml;

mod term_changed;
pub use term_changed::term_changed;
