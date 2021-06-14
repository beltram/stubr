use asserhttp::*;
use surf::get;

use crate::utils::*;

#[async_std::test]
async fn should_always_return_server_header() {
    let srv = given("ping");
    let expected = format!("stubr({})", env!("CARGO_PKG_VERSION"));
    get(&srv.uri()).await
        .expect_status_ok()
        .expect_header("server", expected.as_str());
}

#[async_std::test]
async fn should_always_return_matched_stub_id_header() {
    let srv = given("iso/stub-uuid");
    get(&srv.uri()).await
        .expect_status_ok()
        .expect_header("matched-stub-id", "82d86e05-9ee0-44ca-9a8d-1fc6f719437a");
}

#[async_std::test]
async fn should_not_consider_id_field() {
    let srv = given("iso/ignore-id");
    get(&srv.uri()).await
        .expect_status_ok()
        .expect_header("matched-stub-id", "82d86e05-9ee0-44ca-9a8d-1fc6f719437e");
}

#[async_std::test]
async fn should_support_stubs_without_uuid() {
    let srv = given("iso/no-uuid");
    get(&srv.uri()).await
        .expect_status_ok()
        .expect_header_absent("Matched-Stub-Id");
}