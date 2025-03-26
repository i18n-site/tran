use proto_tran::Filetype;

pub const EXT_LI: &[&str] = &["yml", "md"];

pub fn filetype(fp: &str) -> Filetype {
  if fp.ends_with(".yml") {
    Filetype::Yml
  } else {
    Filetype::Md
  }
}
