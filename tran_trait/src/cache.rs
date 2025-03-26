use std::future::Future;

use aok::{Result, Void};

pub type HashTxt = (Vec<u8>, Vec<u8>);

pub trait Cache {
  fn get(
    &self,
    from_lang: u16,
    to_lang: u16,
    hash_li: &[Vec<u8>],
  ) -> impl Future<Output = Result<Vec<Option<String>>>> + Send;

  fn set(
    &self,
    from_lang: u16,
    to_lang: u16,
    hash_li: &[Vec<u8>],
    li: &[String],
  ) -> impl Future<Output = Void> + Send;

  fn set_src_li(
    &self,
    li: Vec<(
      // lang
      u16,
      // hash txt
      Vec<HashTxt>,
    )>,
  ) -> impl Future<Output = Void> + Send;

  fn set_user(
    &self,
    from_lang: u16,
    to_lang: u16,
    hash_str_li: Vec<(Vec<u8>, &str)>,
  ) -> impl Future<Output = Void> + Send;

  fn src_li(
    &self,
    hash_li: &[(u16, &Vec<u8>)],
  ) -> impl Future<Output = Result<Vec<Option<String>>>> + Send;
}
