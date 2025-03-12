#![feature(let_chains)]

use aok::{OK, Void};
use cmdv::cmdv;
use clap::arg;
use i18n::{Error, tran};

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

#[tokio::main]
async fn main() -> Void {
  if let Some((m, _)) = cmdv!(arg!(-w --workdir [path] "workdir"),) {
    let workdir = m
      .get_one("workdir")
      .map(|s: &String| s.into())
      .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| ".".into()));

    if let Err(e) = tran(&workdir).await {
      if let Some(e) = e.downcast_ref::<Error>() {
        eprintln!("{}", e);
        return OK;
      }
      return Err(e);
    }
  }
  OK
}
