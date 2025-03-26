#![feature(str_as_str)]
#![feature(str_split_remainder)]
#![feature(let_chains)]

genv::s!(I18N_API: String | "http://127.0.0.1:5888/".to_owned());

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

mod scan_tran;
use scan_tran::scan_tran;

mod scan;
use scan::{ScanResult, scan};

mod recv;
use recv::recv;

mod term_changed;
pub use term_changed::term_changed;

mod filetype;
pub use filetype::{EXT_LI, filetype};

mod src;
pub mod state;

mod save;
pub use save::Save;
