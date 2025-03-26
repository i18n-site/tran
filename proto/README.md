# proto_tran : protobuf for i18n

```proto3
syntax = "proto3";

message LangTxt
{
  uint32 lang = 1;
  string txt = 2;
}

enum Filetype {
  Md = 0;
  Yml = 1;
}

message UpdateLi
{
  /* from lang */
  Filetype filetype = 1;
  uint32 lang = 2;
  bytes hash = 3;
  repeated LangTxt li = 4;
}

message RelTxt
{
  string rel = 1;
  string txt = 2;
}

message TranLi
{
  uint32 from_lang = 1;
  repeated uint32 to_lang_li = 2;
  repeated RelTxt li = 3;
}

message Dict
{
  uint32 lang = 1;
  repeated string from_word_li = 2;
  repeated string to_word_li = 3;
}

message Term
{
  uint32 lang = 1;
  repeated string from_word_li = 2;
  repeated string to_word_li = 3;
  repeated Dict dict_li = 4;
}

message Tran
{
  repeated UpdateLi update_li = 1 [ (rust.nullable_field) = false ];
  repeated TranLi tran_li = 2 [ (rust.nullable_field) = false ];
  repeated Term term_li = 3;
}

message TranResult
{
  uint64 id = 1;
  uint32 code = 2;
  string rel = 3;
  uint32 lang = 4;
  string msg = 5;
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
