use surf::get;

use crate::utils::*;

#[async_std::test]
async fn should_url_encode() {
    let srv = given("resp/template/url_encode/encode");
    get(&srv.url()).header("x-raw", "a/b/c").await.unwrap()
        .assert_ok()
        .assert_body_text("a%2Fb%2Fc")
        .assert_content_type_text();
}

#[async_std::test]
async fn should_url_decode() {
    let srv = given("resp/template/url_encode/decode");
    get(&srv.url()).header("x-encoded", "a%2Fb%2Fc").await.unwrap()
        .assert_ok()
        .assert_body_text("a/b/c")
        .assert_content_type_text();
}

#[async_std::test]
async fn should_url_encode_raw() {
    let srv = given("resp/template/url_encode/raw");
    get(&srv.url()).await.unwrap()
        .assert_ok()
        .assert_body_text("a%2Fb%2Fc")
        .assert_content_type_text();
}

#[async_std::test]
async fn should_url_decode_raw() {
    let srv = given("resp/template/url_encode/raw-encoded");
    get(&srv.url()).await.unwrap()
        .assert_ok()
        .assert_body_text("a/b/c")
        .assert_content_type_text();
}