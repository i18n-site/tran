use std::path::Path;

pub fn yml_lang(fp: &Path) -> Option<u32> {
  if let Some(name) = fp.file_name() {
    let name = name.to_string_lossy();
    let len = name.len();
    if name.ends_with(".yml") && len > 4 {
      let lang = &name[..len - 4];
      return lang::by_str(lang).map(|i| i as u32);
    }
  }
  None
}
