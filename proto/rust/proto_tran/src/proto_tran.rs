// @generated, do not edit
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum Filetype {
  Md = 0,
  Yml = 1,
}
impl Filetype {
  pub const KNOWN_VARIANTS: [Filetype; 2] = [Filetype::Md, Filetype::Yml];
}
impl ::std::default::Default for Filetype {
  fn default() -> Self {
    Filetype::Md
  }
}
impl From<Filetype> for i32 {
  fn from(v: Filetype) -> i32 {
    match v {
      Filetype::Md => 0,
      Filetype::Yml => 1,
    }
  }
}
impl ::std::convert::TryFrom<i32> for Filetype {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      0 => Ok(Filetype::Md),
      1 => Ok(Filetype::Yml),
      _ => Err(v),
    }
  }
}
impl ::pb_jelly::ProtoEnum for Filetype {}
impl ::pb_jelly::ClosedProtoEnum for Filetype {
  fn name(self) -> &'static str {
    match self {
      Filetype::Md => "Md",
      Filetype::Yml => "Yml",
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LangTxt {
  pub lang: u32,
  pub txt: ::std::string::String,
}
impl ::std::default::Default for LangTxt {
  fn default() -> Self {
    LangTxt {
      lang: ::std::default::Default::default(),
      txt: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref LangTxt_default: LangTxt = LangTxt::default();
}
impl ::pb_jelly::Message for LangTxt {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "LangTxt",
      full_name: "LangTxt",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "lang",
          full_name: "LangTxt.lang",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "txt",
          full_name: "LangTxt.txt",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    size += ::pb_jelly::helpers::compute_size_scalar::<u32>(
      &self.lang,
      1,
      ::pb_jelly::wire_format::Type::Varint,
    );
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::string::String>(
      &self.txt,
      2,
      ::pb_jelly::wire_format::Type::LengthDelimited,
    );
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    ::pb_jelly::helpers::serialize_scalar::<W, u32>(
      w,
      &self.lang,
      1,
      ::pb_jelly::wire_format::Type::Varint,
    )?;
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::string::String>(
      w,
      &self.txt,
      2,
      ::pb_jelly::wire_format::Type::LengthDelimited,
    )?;
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(
    &mut self,
    mut buf: &mut B,
  ) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, u32>(
            buf,
            typ,
            ::pb_jelly::wire_format::Type::Varint,
            "LangTxt",
            1,
          )?;
          self.lang = val;
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(
            buf, typ, "LangTxt", 2,
          )?;
          self.txt = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for LangTxt {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "lang" => ::pb_jelly::reflection::FieldMut::Value(&mut self.lang),
      "txt" => ::pb_jelly::reflection::FieldMut::Value(&mut self.txt),
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UpdateLi {
  /// from lang
  pub filetype: Filetype,
  pub lang: u32,
  pub hash: ::std::vec::Vec<u8>,
  pub li: ::std::vec::Vec<LangTxt>,
}
impl ::std::default::Default for UpdateLi {
  fn default() -> Self {
    UpdateLi {
      filetype: ::std::default::Default::default(),
      lang: ::std::default::Default::default(),
      hash: ::std::default::Default::default(),
      li: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref UpdateLi_default: UpdateLi = UpdateLi::default();
}
impl ::pb_jelly::Message for UpdateLi {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "UpdateLi",
      full_name: "UpdateLi",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "filetype",
          full_name: "UpdateLi.filetype",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "lang",
          full_name: "UpdateLi.lang",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "hash",
          full_name: "UpdateLi.hash",
          index: 2,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "li",
          full_name: "UpdateLi.li",
          index: 3,
          number: 4,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
      ],
      oneofs: &[],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    size += ::pb_jelly::helpers::compute_size_scalar::<Filetype>(
      &self.filetype,
      1,
      ::pb_jelly::wire_format::Type::Varint,
    );
    size += ::pb_jelly::helpers::compute_size_scalar::<u32>(
      &self.lang,
      2,
      ::pb_jelly::wire_format::Type::Varint,
    );
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::vec::Vec<u8>>(
      &self.hash,
      3,
      ::pb_jelly::wire_format::Type::LengthDelimited,
    );
    for val in &self.li {
      size += ::pb_jelly::helpers::compute_size_field::<LangTxt>(
        val,
        4,
        ::pb_jelly::wire_format::Type::LengthDelimited,
      );
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    ::pb_jelly::helpers::serialize_scalar::<W, Filetype>(
      w,
      &self.filetype,
      1,
      ::pb_jelly::wire_format::Type::Varint,
    )?;
    ::pb_jelly::helpers::serialize_scalar::<W, u32>(
      w,
      &self.lang,
      2,
      ::pb_jelly::wire_format::Type::Varint,
    )?;
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::vec::Vec<u8>>(
      w,
      &self.hash,
      3,
      ::pb_jelly::wire_format::Type::LengthDelimited,
    )?;
    for val in &self.li {
      ::pb_jelly::helpers::serialize_field::<W, LangTxt>(
        w,
        val,
        4,
        ::pb_jelly::wire_format::Type::LengthDelimited,
      )?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(
    &mut self,
    mut buf: &mut B,
  ) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, Filetype>(
            buf,
            typ,
            ::pb_jelly::wire_format::Type::Varint,
            "UpdateLi",
            1,
          )?;
          self.filetype = val;
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, u32>(
            buf,
            typ,
            ::pb_jelly::wire_format::Type::Varint,
            "UpdateLi",
            2,
          )?;
          self.lang = val;
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::vec::Vec<u8>>(
            buf, typ, "UpdateLi", 3,
          )?;
          self.hash = val;
        }
        4 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, LangTxt>(
            buf, typ, "UpdateLi", 4,
          )?;
          self.li.push(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for UpdateLi {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "filetype" => ::pb_jelly::reflection::FieldMut::Value(&mut self.filetype),
      "lang" => ::pb_jelly::reflection::FieldMut::Value(&mut self.lang),
      "hash" => ::pb_jelly::reflection::FieldMut::Value(&mut self.hash),
      "li" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RelTxt {
  pub rel: ::std::string::String,
  pub txt: ::std::string::String,
}
impl ::std::default::Default for RelTxt {
  fn default() -> Self {
    RelTxt {
      rel: ::std::default::Default::default(),
      txt: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref RelTxt_default: RelTxt = RelTxt::default();
}
impl ::pb_jelly::Message for RelTxt {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "RelTxt",
      full_name: "RelTxt",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "rel",
          full_name: "RelTxt.rel",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "txt",
          full_name: "RelTxt.txt",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::string::String>(
      &self.rel,
      1,
      ::pb_jelly::wire_format::Type::LengthDelimited,
    );
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::string::String>(
      &self.txt,
      2,
      ::pb_jelly::wire_format::Type::LengthDelimited,
    );
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::string::String>(
      w,
      &self.rel,
      1,
      ::pb_jelly::wire_format::Type::LengthDelimited,
    )?;
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::string::String>(
      w,
      &self.txt,
      2,
      ::pb_jelly::wire_format::Type::LengthDelimited,
    )?;
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(
    &mut self,
    mut buf: &mut B,
  ) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(
            buf, typ, "RelTxt", 1,
          )?;
          self.rel = val;
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(
            buf, typ, "RelTxt", 2,
          )?;
          self.txt = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for RelTxt {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "rel" => ::pb_jelly::reflection::FieldMut::Value(&mut self.rel),
      "txt" => ::pb_jelly::reflection::FieldMut::Value(&mut self.txt),
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TranLi {
  pub from_lang: u32,
  pub to_lang_li: ::std::vec::Vec<u32>,
  pub li: ::std::vec::Vec<RelTxt>,
}
impl ::std::default::Default for TranLi {
  fn default() -> Self {
    TranLi {
      from_lang: ::std::default::Default::default(),
      to_lang_li: ::std::default::Default::default(),
      li: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref TranLi_default: TranLi = TranLi::default();
}
impl ::pb_jelly::Message for TranLi {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "TranLi",
      full_name: "TranLi",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "from_lang",
          full_name: "TranLi.from_lang",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "to_lang_li",
          full_name: "TranLi.to_lang_li",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "li",
          full_name: "TranLi.li",
          index: 2,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
      ],
      oneofs: &[],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    size += ::pb_jelly::helpers::compute_size_scalar::<u32>(
      &self.from_lang,
      1,
      ::pb_jelly::wire_format::Type::Varint,
    );
    if !self.to_lang_li.is_empty() {
      let mut to_lang_li_size = 0usize;
      for val in &self.to_lang_li {
        to_lang_li_size += ::pb_jelly::Message::compute_size(val);
      }
      size += ::pb_jelly::wire_format::serialized_length(2);
      size += ::pb_jelly::varint::serialized_length(to_lang_li_size as u64);
      size += to_lang_li_size;
    }
    for val in &self.li {
      size += ::pb_jelly::helpers::compute_size_field::<RelTxt>(
        val,
        3,
        ::pb_jelly::wire_format::Type::LengthDelimited,
      );
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    ::pb_jelly::helpers::serialize_scalar::<W, u32>(
      w,
      &self.from_lang,
      1,
      ::pb_jelly::wire_format::Type::Varint,
    )?;
    if !self.to_lang_li.is_empty() {
      let mut size = 0usize;
      for val in &self.to_lang_li {
        size += ::pb_jelly::Message::compute_size(val);
      }
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      ::pb_jelly::varint::write(size as u64, w)?;
      for val in &self.to_lang_li {
        ::pb_jelly::Message::serialize(val, w)?;
      }
    }
    for val in &self.li {
      ::pb_jelly::helpers::serialize_field::<W, RelTxt>(
        w,
        val,
        3,
        ::pb_jelly::wire_format::Type::LengthDelimited,
      )?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(
    &mut self,
    mut buf: &mut B,
  ) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, u32>(
            buf,
            typ,
            ::pb_jelly::wire_format::Type::Varint,
            "TranLi",
            1,
          )?;
          self.from_lang = val;
        }
        2 => {
          ::pb_jelly::helpers::deserialize_packed::<B, u32>(
            buf,
            typ,
            ::pb_jelly::wire_format::Type::Varint,
            "TranLi",
            2,
            &mut self.to_lang_li,
          )?;
        }
        3 => {
          let val =
            ::pb_jelly::helpers::deserialize_length_delimited::<B, RelTxt>(buf, typ, "TranLi", 3)?;
          self.li.push(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for TranLi {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "from_lang" => ::pb_jelly::reflection::FieldMut::Value(&mut self.from_lang),
      "to_lang_li" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "li" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Dict {
  pub lang: u32,
  pub from_word_li: ::std::vec::Vec<::std::string::String>,
  pub to_word_li: ::std::vec::Vec<::std::string::String>,
}
impl ::std::default::Default for Dict {
  fn default() -> Self {
    Dict {
      lang: ::std::default::Default::default(),
      from_word_li: ::std::default::Default::default(),
      to_word_li: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref Dict_default: Dict = Dict::default();
}
impl ::pb_jelly::Message for Dict {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "Dict",
      full_name: "Dict",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "lang",
          full_name: "Dict.lang",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "from_word_li",
          full_name: "Dict.from_word_li",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "to_word_li",
          full_name: "Dict.to_word_li",
          index: 2,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
      ],
      oneofs: &[],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    size += ::pb_jelly::helpers::compute_size_scalar::<u32>(
      &self.lang,
      1,
      ::pb_jelly::wire_format::Type::Varint,
    );
    for val in &self.from_word_li {
      size += ::pb_jelly::helpers::compute_size_field::<::std::string::String>(
        val,
        2,
        ::pb_jelly::wire_format::Type::LengthDelimited,
      );
    }
    for val in &self.to_word_li {
      size += ::pb_jelly::helpers::compute_size_field::<::std::string::String>(
        val,
        3,
        ::pb_jelly::wire_format::Type::LengthDelimited,
      );
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    ::pb_jelly::helpers::serialize_scalar::<W, u32>(
      w,
      &self.lang,
      1,
      ::pb_jelly::wire_format::Type::Varint,
    )?;
    for val in &self.from_word_li {
      ::pb_jelly::helpers::serialize_field::<W, ::std::string::String>(
        w,
        val,
        2,
        ::pb_jelly::wire_format::Type::LengthDelimited,
      )?;
    }
    for val in &self.to_word_li {
      ::pb_jelly::helpers::serialize_field::<W, ::std::string::String>(
        w,
        val,
        3,
        ::pb_jelly::wire_format::Type::LengthDelimited,
      )?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(
    &mut self,
    mut buf: &mut B,
  ) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, u32>(
            buf,
            typ,
            ::pb_jelly::wire_format::Type::Varint,
            "Dict",
            1,
          )?;
          self.lang = val;
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(
            buf, typ, "Dict", 2,
          )?;
          self.from_word_li.push(val);
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(
            buf, typ, "Dict", 3,
          )?;
          self.to_word_li.push(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for Dict {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "lang" => ::pb_jelly::reflection::FieldMut::Value(&mut self.lang),
      "from_word_li" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "to_word_li" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Term {
  pub lang: u32,
  pub from_word_li: ::std::vec::Vec<::std::string::String>,
  pub to_word_li: ::std::vec::Vec<::std::string::String>,
  pub dict_li: ::std::vec::Vec<Dict>,
}
impl ::std::default::Default for Term {
  fn default() -> Self {
    Term {
      lang: ::std::default::Default::default(),
      from_word_li: ::std::default::Default::default(),
      to_word_li: ::std::default::Default::default(),
      dict_li: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref Term_default: Term = Term::default();
}
impl ::pb_jelly::Message for Term {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "Term",
      full_name: "Term",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "lang",
          full_name: "Term.lang",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "from_word_li",
          full_name: "Term.from_word_li",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "to_word_li",
          full_name: "Term.to_word_li",
          index: 2,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "dict_li",
          full_name: "Term.dict_li",
          index: 3,
          number: 4,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
      ],
      oneofs: &[],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    size += ::pb_jelly::helpers::compute_size_scalar::<u32>(
      &self.lang,
      1,
      ::pb_jelly::wire_format::Type::Varint,
    );
    for val in &self.from_word_li {
      size += ::pb_jelly::helpers::compute_size_field::<::std::string::String>(
        val,
        2,
        ::pb_jelly::wire_format::Type::LengthDelimited,
      );
    }
    for val in &self.to_word_li {
      size += ::pb_jelly::helpers::compute_size_field::<::std::string::String>(
        val,
        3,
        ::pb_jelly::wire_format::Type::LengthDelimited,
      );
    }
    for val in &self.dict_li {
      size += ::pb_jelly::helpers::compute_size_field::<Dict>(
        val,
        4,
        ::pb_jelly::wire_format::Type::LengthDelimited,
      );
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    ::pb_jelly::helpers::serialize_scalar::<W, u32>(
      w,
      &self.lang,
      1,
      ::pb_jelly::wire_format::Type::Varint,
    )?;
    for val in &self.from_word_li {
      ::pb_jelly::helpers::serialize_field::<W, ::std::string::String>(
        w,
        val,
        2,
        ::pb_jelly::wire_format::Type::LengthDelimited,
      )?;
    }
    for val in &self.to_word_li {
      ::pb_jelly::helpers::serialize_field::<W, ::std::string::String>(
        w,
        val,
        3,
        ::pb_jelly::wire_format::Type::LengthDelimited,
      )?;
    }
    for val in &self.dict_li {
      ::pb_jelly::helpers::serialize_field::<W, Dict>(
        w,
        val,
        4,
        ::pb_jelly::wire_format::Type::LengthDelimited,
      )?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(
    &mut self,
    mut buf: &mut B,
  ) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, u32>(
            buf,
            typ,
            ::pb_jelly::wire_format::Type::Varint,
            "Term",
            1,
          )?;
          self.lang = val;
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(
            buf, typ, "Term", 2,
          )?;
          self.from_word_li.push(val);
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(
            buf, typ, "Term", 3,
          )?;
          self.to_word_li.push(val);
        }
        4 => {
          let val =
            ::pb_jelly::helpers::deserialize_length_delimited::<B, Dict>(buf, typ, "Term", 4)?;
          self.dict_li.push(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for Term {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "lang" => ::pb_jelly::reflection::FieldMut::Value(&mut self.lang),
      "from_word_li" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "to_word_li" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "dict_li" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Tran {
  pub update_li: ::std::vec::Vec<UpdateLi>,
  pub tran_li: ::std::vec::Vec<TranLi>,
  pub term_li: ::std::vec::Vec<Term>,
}
impl ::std::default::Default for Tran {
  fn default() -> Self {
    Tran {
      update_li: ::std::default::Default::default(),
      tran_li: ::std::default::Default::default(),
      term_li: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref Tran_default: Tran = Tran::default();
}
impl ::pb_jelly::Message for Tran {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "Tran",
      full_name: "Tran",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "update_li",
          full_name: "Tran.update_li",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "tran_li",
          full_name: "Tran.tran_li",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "term_li",
          full_name: "Tran.term_li",
          index: 2,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
      ],
      oneofs: &[],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    for val in &self.update_li {
      size += ::pb_jelly::helpers::compute_size_field::<UpdateLi>(
        val,
        1,
        ::pb_jelly::wire_format::Type::LengthDelimited,
      );
    }
    for val in &self.tran_li {
      size += ::pb_jelly::helpers::compute_size_field::<TranLi>(
        val,
        2,
        ::pb_jelly::wire_format::Type::LengthDelimited,
      );
    }
    for val in &self.term_li {
      size += ::pb_jelly::helpers::compute_size_field::<Term>(
        val,
        3,
        ::pb_jelly::wire_format::Type::LengthDelimited,
      );
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.update_li {
      ::pb_jelly::helpers::serialize_field::<W, UpdateLi>(
        w,
        val,
        1,
        ::pb_jelly::wire_format::Type::LengthDelimited,
      )?;
    }
    for val in &self.tran_li {
      ::pb_jelly::helpers::serialize_field::<W, TranLi>(
        w,
        val,
        2,
        ::pb_jelly::wire_format::Type::LengthDelimited,
      )?;
    }
    for val in &self.term_li {
      ::pb_jelly::helpers::serialize_field::<W, Term>(
        w,
        val,
        3,
        ::pb_jelly::wire_format::Type::LengthDelimited,
      )?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(
    &mut self,
    mut buf: &mut B,
  ) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val =
            ::pb_jelly::helpers::deserialize_length_delimited::<B, UpdateLi>(buf, typ, "Tran", 1)?;
          self.update_li.push(val);
        }
        2 => {
          let val =
            ::pb_jelly::helpers::deserialize_length_delimited::<B, TranLi>(buf, typ, "Tran", 2)?;
          self.tran_li.push(val);
        }
        3 => {
          let val =
            ::pb_jelly::helpers::deserialize_length_delimited::<B, Term>(buf, typ, "Tran", 3)?;
          self.term_li.push(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for Tran {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "update_li" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "tran_li" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "term_li" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TranResult {
  pub id: u64,
  pub code: u32,
  pub rel: ::std::string::String,
  pub lang: u32,
  pub msg: ::std::string::String,
}

impl ::std::default::Default for TranResult {
  fn default() -> Self {
    TranResult {
      id: ::std::default::Default::default(),
      code: ::std::default::Default::default(),
      rel: ::std::default::Default::default(),
      lang: ::std::default::Default::default(),
      msg: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref TranResult_default: TranResult = TranResult::default();
}
impl ::pb_jelly::Message for TranResult {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "TranResult",
      full_name: "TranResult",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "id",
          full_name: "TranResult.id",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "code",
          full_name: "TranResult.code",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "rel",
          full_name: "TranResult.rel",
          index: 2,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "lang",
          full_name: "TranResult.lang",
          index: 3,
          number: 4,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "msg",
          full_name: "TranResult.msg",
          index: 4,
          number: 5,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    size += ::pb_jelly::helpers::compute_size_scalar::<u64>(
      &self.id,
      1,
      ::pb_jelly::wire_format::Type::Varint,
    );
    size += ::pb_jelly::helpers::compute_size_scalar::<u32>(
      &self.code,
      2,
      ::pb_jelly::wire_format::Type::Varint,
    );
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::string::String>(
      &self.rel,
      3,
      ::pb_jelly::wire_format::Type::LengthDelimited,
    );
    size += ::pb_jelly::helpers::compute_size_scalar::<u32>(
      &self.lang,
      4,
      ::pb_jelly::wire_format::Type::Varint,
    );
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::string::String>(
      &self.msg,
      5,
      ::pb_jelly::wire_format::Type::LengthDelimited,
    );
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    ::pb_jelly::helpers::serialize_scalar::<W, u64>(
      w,
      &self.id,
      1,
      ::pb_jelly::wire_format::Type::Varint,
    )?;
    ::pb_jelly::helpers::serialize_scalar::<W, u32>(
      w,
      &self.code,
      2,
      ::pb_jelly::wire_format::Type::Varint,
    )?;
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::string::String>(
      w,
      &self.rel,
      3,
      ::pb_jelly::wire_format::Type::LengthDelimited,
    )?;
    ::pb_jelly::helpers::serialize_scalar::<W, u32>(
      w,
      &self.lang,
      4,
      ::pb_jelly::wire_format::Type::Varint,
    )?;
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::string::String>(
      w,
      &self.msg,
      5,
      ::pb_jelly::wire_format::Type::LengthDelimited,
    )?;
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(
    &mut self,
    mut buf: &mut B,
  ) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, u64>(
            buf,
            typ,
            ::pb_jelly::wire_format::Type::Varint,
            "TranResult",
            1,
          )?;
          self.id = val;
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, u32>(
            buf,
            typ,
            ::pb_jelly::wire_format::Type::Varint,
            "TranResult",
            2,
          )?;
          self.code = val;
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(
            buf,
            typ,
            "TranResult",
            3,
          )?;
          self.rel = val;
        }
        4 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, u32>(
            buf,
            typ,
            ::pb_jelly::wire_format::Type::Varint,
            "TranResult",
            4,
          )?;
          self.lang = val;
        }
        5 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(
            buf,
            typ,
            "TranResult",
            5,
          )?;
          self.msg = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for TranResult {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "id" => ::pb_jelly::reflection::FieldMut::Value(&mut self.id),
      "code" => ::pb_jelly::reflection::FieldMut::Value(&mut self.code),
      "rel" => ::pb_jelly::reflection::FieldMut::Value(&mut self.rel),
      "lang" => ::pb_jelly::reflection::FieldMut::Value(&mut self.lang),
      "msg" => ::pb_jelly::reflection::FieldMut::Value(&mut self.msg),
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}
