use surf::post;

use crate::utils::*;

#[async_std::test]
async fn should_encode_into_base64() {
    let srv = given("resp/template/base64/encode");
    post(&srv.url()).body("abcd").await.unwrap()
        .assert_ok()
        .assert_body_text("YWJjZA==")
        .assert_content_type_text();
}

#[async_std::test]
async fn should_encode_raw_into_base64() {
    let srv = given("resp/template/base64/encode-raw");
    post(&srv.url()).await.unwrap()
        .assert_ok()
        .assert_body_text("aGVsbG8=")
        .assert_content_type_text();
}

#[async_std::test]
async fn should_encode_into_base64_without_padding() {
    let srv = given("resp/template/base64/encode-no-padding");
    post(&srv.url()).body("abcd").await.unwrap()
        .assert_ok()
        .assert_body_text("YWJjZA")
        .assert_content_type_text();
}

#[async_std::test]
async fn should_encode_into_base64_with_padding() {
    let srv = given("resp/template/base64/encode-with-padding");
    post(&srv.url()).body("abcd").await.unwrap()
        .assert_ok()
        .assert_body_text("YWJjZA==")
        .assert_content_type_text();
}

#[async_std::test]
async fn should_decode_from_base64() {
    let srv = given("resp/template/base64/decode");
    post(&srv.url()).body("YWJjZA==").await.unwrap()
        .assert_ok()
        .assert_body_text("abcd")
        .assert_content_type_text();
}

#[async_std::test]
async fn should_encode_into_base64_when_decode_false() {
    let srv = given("resp/template/base64/not-decode");
    post(&srv.url()).body("abcd").await.unwrap()
        .assert_ok()
        .assert_body_text("YWJjZA==")
        .assert_content_type_text();
}