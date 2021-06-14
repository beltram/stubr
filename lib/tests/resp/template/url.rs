use asserhttp::*;
use surf::get;

use stubr::Config;

use crate::utils::*;

#[async_std::test]
async fn should_template_request_path() {
    let srv = given("resp/template/url/path");
    get(&srv.path_query("/api/path", "name", "beltram")).await
        .expect_status_ok()
        .expect_body_text_eq("/api/path")
        .expect_content_type_text();
}

#[async_std::test]
async fn should_template_request_url() {
    let srv = given("resp/template/url/url");
    get(&srv.path_query("/api/path", "name", "beltram")).await
        .expect_status_ok()
        .expect_body_text_eq("/api/path?name&#x3D;beltram")
        .expect_content_type_text();
}

#[async_std::test]
#[ignore] // need to fix this in wiremock-rs
async fn should_template_request_port() {
    let cfg = Config { port: Some(59_000), ..Default::default() };
    let srv = Stubr::start_with("tests/stubs/resp/template/url/port.json", cfg).await;
    get(&srv.path("/api/port")).await
        .expect_status_ok()
        .expect_body_text_eq("59000")
        .expect_content_type_text();
}

#[async_std::test]
async fn should_template_request_method() {
    let srv = given("resp/template/url/method");
    surf::get(&srv.uri()).await
        .expect_status_ok()
        .expect_content_type_text()
        .expect_body_text_eq("GET");
    surf::post(&srv.uri()).await
        .expect_status_ok()
        .expect_content_type_text()
        .expect_body_text_eq("POST");
    surf::put(&srv.uri()).await
        .expect_status_ok()
        .expect_content_type_text()
        .expect_body_text_eq("PUT");
    surf::delete(&srv.uri()).await
        .expect_status_ok()
        .expect_content_type_text()
        .expect_body_text_eq("DELETE");
    surf::patch(&srv.uri()).await
        .expect_status_ok()
        .expect_content_type_text()
        .expect_body_text_eq("PATCH");
    surf::options(&srv.uri()).await
        .expect_status_ok()
        .expect_content_type_text()
        .expect_body_text_eq("OPTIONS");
    surf::trace(&srv.uri()).await
        .expect_status_ok()
        .expect_content_type_text()
        .expect_body_text_eq("TRACE");
}

#[async_std::test]
async fn should_template_request_path_segments() {
    let srv = given("resp/template/url/path-segments");
    get(&srv.path("/one/two/three")).await
        .expect_status_ok()
        .expect_body_text_eq("two")
        .expect_content_type_text();
}
