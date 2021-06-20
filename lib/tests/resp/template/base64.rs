use asserhttp::*;
use surf::post;

#[async_std::test]
#[stubr::mock("resp/template/base64/encode.json")]
async fn should_encode_into_base64() {
    post(stubr.uri()).body("abcd").await
        .expect_status_ok()
        .expect_body_text_eq("YWJjZA==")
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/base64/encode-raw.json")]
async fn should_encode_raw_into_base64() {
    post(stubr.uri()).await
        .expect_status_ok()
        .expect_body_text_eq("aGVsbG8=")
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/base64/encode-no-padding.json")]
async fn should_encode_into_base64_without_padding() {
    post(stubr.uri()).body("abcd").await
        .expect_status_ok()
        .expect_body_text_eq("YWJjZA")
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/base64/encode-with-padding.json")]
async fn should_encode_into_base64_with_padding() {
    post(stubr.uri()).body("abcd").await
        .expect_status_ok()
        .expect_body_text_eq("YWJjZA==")
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/base64/decode.json")]
async fn should_decode_from_base64() {
    post(stubr.uri()).body("YWJjZA==").await
        .expect_status_ok()
        .expect_body_text_eq("abcd")
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/base64/not-decode.json")]
async fn should_encode_into_base64_when_decode_false() {
    post(stubr.uri()).body("abcd").await
        .expect_status_ok()
        .expect_body_text_eq("YWJjZA==")
        .expect_content_type_text();
}