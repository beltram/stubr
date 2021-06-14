use asserhttp::*;
use surf::get;

use crate::utils::*;

#[async_std::test]
async fn should_url_encode() {
    let srv = given("resp/template/url_encode/encode");
    get(&srv.uri()).header("x-raw", "a/b/c").await
        .expect_status_ok()
        .expect_body_text_eq("a%2Fb%2Fc")
        .expect_content_type_text();
}

#[async_std::test]
async fn should_url_decode() {
    let srv = given("resp/template/url_encode/decode");
    get(&srv.uri()).header("x-encoded", "a%2Fb%2Fc").await
        .expect_status_ok()
        .expect_body_text_eq("a/b/c")
        .expect_content_type_text();
}

#[async_std::test]
async fn should_url_encode_raw() {
    let srv = given("resp/template/url_encode/raw");
    get(&srv.uri()).await
        .expect_status_ok()
        .expect_body_text_eq("a%2Fb%2Fc")
        .expect_content_type_text();
}

#[async_std::test]
async fn should_url_decode_raw() {
    let srv = given("resp/template/url_encode/raw-encoded");
    get(&srv.uri()).await
        .expect_status_ok()
        .expect_body_text_eq("a/b/c")
        .expect_content_type_text();
}