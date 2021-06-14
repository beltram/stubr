use asserhttp::*;
use surf::post;

use crate::utils::*;

#[async_std::test]
async fn should_encode_into_base64() {
    let srv = given("resp/template/base64/encode");
    post(&srv.uri()).body("abcd").await
        .expect_status_ok()
        .expect_body_text_eq("YWJjZA==")
        .expect_content_type_text();
}

#[async_std::test]
async fn should_encode_raw_into_base64() {
    let srv = given("resp/template/base64/encode-raw");
    post(&srv.uri()).await
        .expect_status_ok()
        .expect_body_text_eq("aGVsbG8=")
        .expect_content_type_text();
}

#[async_std::test]
async fn should_encode_into_base64_without_padding() {
    let srv = given("resp/template/base64/encode-no-padding");
    post(&srv.uri()).body("abcd").await
        .expect_status_ok()
        .expect_body_text_eq("YWJjZA")
        .expect_content_type_text();
}

#[async_std::test]
async fn should_encode_into_base64_with_padding() {
    let srv = given("resp/template/base64/encode-with-padding");
    post(&srv.uri()).body("abcd").await
        .expect_status_ok()
        .expect_body_text_eq("YWJjZA==")
        .expect_content_type_text();
}

#[async_std::test]
async fn should_decode_from_base64() {
    let srv = given("resp/template/base64/decode");
    post(&srv.uri()).body("YWJjZA==").await
        .expect_status_ok()
        .expect_body_text_eq("abcd")
        .expect_content_type_text();
}

#[async_std::test]
async fn should_encode_into_base64_when_decode_false() {
    let srv = given("resp/template/base64/not-decode");
    post(&srv.uri()).body("abcd").await
        .expect_status_ok()
        .expect_body_text_eq("YWJjZA==")
        .expect_content_type_text();
}