use asserhttp::*;
use surf::get;

#[async_std::test]
#[stubr::mock]
async fn should_always_return_server_header() {
    let expected = format!("stubr({})", env!("CARGO_PKG_VERSION"));
    get(stubr.uri())
        .await
        .expect_status_ok()
        .expect_header("server", expected.as_str());
}

#[async_std::test]
#[stubr::mock("iso/stub-uuid.json")]
async fn should_always_return_matched_stub_id_header() {
    get(stubr.uri())
        .await
        .expect_status_ok()
        .expect_header("matched-stub-id", "82d86e05-9ee0-44ca-9a8d-1fc6f719437a");
}

#[async_std::test]
#[stubr::mock("iso/ignore-id.json")]
async fn should_not_consider_id_field() {
    get(stubr.uri())
        .await
        .expect_status_ok()
        .expect_header("matched-stub-id", "82d86e05-9ee0-44ca-9a8d-1fc6f719437e");
}

#[async_std::test]
#[stubr::mock("iso/no-uuid.json")]
async fn should_support_stubs_without_uuid() {
    get(stubr.uri())
        .await
        .expect_status_ok()
        .expect_header_absent("Matched-Stub-Id");
}
