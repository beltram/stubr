use asserhttp::*;
use surf::get;

#[async_std::test]
#[stubr::mock("resp/template/url_encode/encode.json")]
async fn should_url_encode() {
    get(stubr.uri()).header("x-raw", "a/b/c").await
        .expect_status_ok()
        .expect_body_text_eq("a%2Fb%2Fc")
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/url_encode/decode.json")]
async fn should_url_decode() {
    get(stubr.uri()).header("x-encoded", "a%2Fb%2Fc").await
        .expect_status_ok()
        .expect_body_text_eq("a/b/c")
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/url_encode/raw.json")]
async fn should_url_encode_raw() {
    get(stubr.uri()).await
        .expect_status_ok()
        .expect_body_text_eq("a%2Fb%2Fc")
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/url_encode/raw-encoded.json")]
async fn should_url_decode_raw() {
    get(stubr.uri()).await
        .expect_status_ok()
        .expect_body_text_eq("a/b/c")
        .expect_content_type_text();
}