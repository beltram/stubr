use surf::get;

use crate::utils::*;

#[async_std::test]
async fn should_always_return_server_header() {
    let srv = given("ping");
    let expected = format!("stubr({})", env!("CARGO_PKG_VERSION"));
    get(&srv.uri()).await.unwrap()
        .assert_ok()
        .assert_header("Server", &expected);
}

#[async_std::test]
async fn should_always_return_matched_stub_id_header() {
    let srv = given("iso/stub-uuid");
    let expected = "82d86e05-9ee0-44ca-9a8d-1fc6f719437a";
    get(&srv.uri()).await.unwrap()
        .assert_ok()
        .assert_header("Matched-Stub-Id", &expected);
}

#[async_std::test]
async fn should_not_consider_id_field() {
    let srv = given("iso/ignore-id");
    let expected = "82d86e05-9ee0-44ca-9a8d-1fc6f719437e";
    get(&srv.uri()).await.unwrap()
        .assert_ok()
        .assert_header("Matched-Stub-Id", &expected);
}

#[async_std::test]
async fn should_support_stubs_without_uuid() {
    let srv = given("iso/no-uuid");
    get(&srv.uri()).await.unwrap()
        .assert_ok()
        .assert_no_header("Matched-Stub-Id");
}