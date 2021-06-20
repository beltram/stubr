use asserhttp::*;
use surf::get;

use stubr::Config;

use crate::utils::*;

#[async_std::test]
#[stubr::mock("resp/template/url/path.json")]
async fn should_template_request_path() {
    get(stubr.path_query("/api/path", "name", "beltram")).await
        .expect_status_ok()
        .expect_body_text_eq("/api/path")
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/url/url.json")]
async fn should_template_request_url() {
    get(stubr.path_query("/api/path", "name", "beltram")).await
        .expect_status_ok()
        .expect_body_text_eq("/api/path?name&#x3D;beltram")
        .expect_content_type_text();
}

#[async_std::test]
#[ignore] // need to fix this in wiremock-rs
async fn should_template_request_port() {
    let cfg = Config { port: Some(59_000), ..Default::default() };
    let stubr = Stubr::start_with("tests/stubs/resp/template/url/port.json", cfg).await;
    get(stubr.path("/api/port")).await
        .expect_status_ok()
        .expect_body_text_eq("59000")
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/url/method.json")]
async fn should_template_request_method() {
    surf::get(stubr.uri()).await
        .expect_status_ok()
        .expect_content_type_text()
        .expect_body_text_eq("GET");
    surf::post(stubr.uri()).await
        .expect_status_ok()
        .expect_content_type_text()
        .expect_body_text_eq("POST");
    surf::put(stubr.uri()).await
        .expect_status_ok()
        .expect_content_type_text()
        .expect_body_text_eq("PUT");
    surf::delete(stubr.uri()).await
        .expect_status_ok()
        .expect_content_type_text()
        .expect_body_text_eq("DELETE");
    surf::patch(stubr.uri()).await
        .expect_status_ok()
        .expect_content_type_text()
        .expect_body_text_eq("PATCH");
    surf::options(stubr.uri()).await
        .expect_status_ok()
        .expect_content_type_text()
        .expect_body_text_eq("OPTIONS");
    surf::trace(stubr.uri()).await
        .expect_status_ok()
        .expect_content_type_text()
        .expect_body_text_eq("TRACE");
}

#[async_std::test]
#[stubr::mock("resp/template/url/path-segments.json")]
async fn should_template_request_path_segments() {
    get(stubr.path("/one/two/three")).await
        .expect_status_ok()
        .expect_body_text_eq("two")
        .expect_content_type_text();
}
