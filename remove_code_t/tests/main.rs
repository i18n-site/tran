use remove_code_t::remove_code_t;

#[static_init::constructor(0)]
extern "C" fn _loginit() {
  loginit::init();
}

// #[tokio::test]
// async fn test_async() -> Void {
//   info!("async {}", 123456);
//   OK
// }

#[test]
fn test_simple_pair() {
  assert_eq!(
    remove_code_t(r#"Hello <code t>World</code>!"#),
    "Hello World!"
  );
  assert_eq!(
    remove_code_t(r#"Hello <code t>World</code> <code t>World</code>"#),
    "Hello World World"
  );
}
