#![feature(let_chains)]

const START_TAG: &str = "<code t>";
const END_TAG: &str = "</code>";
const START_TAG_LEN: usize = START_TAG.len();
const END_TAG_LEN: usize = END_TAG.len();

pub fn remove_code_t(htm: impl AsRef<str>) -> String {
  let htm = htm.as_ref();
  let mut result = String::with_capacity(htm.len());
  let mut pre = 0;
  let htm_len = htm.len();

  loop {
    if let Some(start) = htm[pre..].find(START_TAG) {
      let start = pre + start;
      let offset = start + START_TAG_LEN;
      if let Some(end) = htm[offset..].find(END_TAG) {
        result += &htm[pre..start];
        let end = offset + end;
        result += &htmlize::unescape(&htm[offset..end]);
        pre = end + END_TAG_LEN;
        continue;
      }
    }
    break;
  }

  if pre < htm_len {
    result += &htm[pre..];
  }
  result
}
