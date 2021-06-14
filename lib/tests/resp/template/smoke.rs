use asserhttp::*;
use serde_json::json;
use surf::get;

use crate::utils::*;

#[async_std::test]
async fn should_template_in_body() {
    let srv = given("resp/template/smoke/in-body");
    get(&srv.path("/api/path")).await
        .expect_status_ok()
        .expect_body_text_eq("/api/path")
        .expect_content_type_text();
}

#[async_std::test]
async fn should_not_template_in_body_when_no_placeholder() {
    let srv = given("resp/template/smoke/in-body-unused");
    get(&srv.path("/api/path")).await
        .expect_status_ok()
        .expect_body_text_eq("any")
        .expect_content_type_text();
}

#[async_std::test]
async fn should_template_in_json_body() {
    let srv = given("resp/template/smoke/in-json-body");
    get(&srv.path("/api/path")).await
        .expect_status_ok()
        .expect_body_json_eq(json!({"path": "/api/path"}))
        .expect_content_type_json();
}

#[async_std::test]
async fn should_not_template_in_json_body_when_no_placeholder() {
    let srv = given("resp/template/smoke/in-json-body-unused");
    get(&srv.path("/api/path")).await
        .expect_status_ok()
        .expect_body_json_eq(json!({"path": "any"}))
        .expect_content_type_json();
}

#[async_std::test]
async fn should_template_in_json_file_body() {
    let srv = given("resp/template/smoke/in-body-json-file");
    get(&srv.path("/api/path")).await
        .expect_status_ok()
        .expect_body_json_eq(json!({"path": "/api/path"}))
        .expect_content_type_json();
}

#[async_std::test]
async fn should_not_template_in_json_file_body_when_no_placeholder() {
    let srv = given("resp/template/smoke/in-body-json-file-unused");
    get(&srv.path("/api/path")).await
        .expect_status_ok()
        .expect_body_json_eq(json!({"path": "any"}))
        .expect_content_type_json();
}

#[async_std::test]
async fn should_template_in_text_file_body() {
    let srv = given("resp/template/smoke/in-body-text-file");
    get(&srv.path("/api/path")).await
        .expect_status_ok()
        .expect_body_text_eq("Lorem ipsum /api/path dolor sit amet")
        .expect_content_type_text();
}

#[async_std::test]
async fn should_not_template_in_text_file_body_when_no_placeholder() {
    let srv = given("resp/template/smoke/in-body-text-file-unused");
    get(&srv.path("/api/path")).await
        .expect_status_ok()
        .expect_body_text_eq("Lorem ipsum any dolor sit amet")
        .expect_content_type_text();
}

#[async_std::test]
async fn should_template_in_response_headers() {
    let srv = given("resp/template/smoke/in-header");
    get(&srv.path("/api/path")).await
        .expect_status_ok()
        .expect_header("x-req-path", "/api/path");
}

#[async_std::test]
async fn should_template_in_many_response_headers() {
    let srv = given("resp/template/smoke/in-many-header");
    get(&srv.path("/api/path")).await
        .expect_status_ok()
        .expect_header("x-1", "/api/path")
        .expect_header("x-2", "GET");
}

#[async_std::test]
async fn should_not_template_in_response_headers_when_no_placeholder() {
    let srv = given("resp/template/smoke/in-header-unused");
    get(&srv.path("/api/path")).await
        .expect_status_ok()
        .expect_header("x-req-path", "/the/path");
}

#[async_std::test]
async fn stubs_should_be_isolated() {
    let srv = Stubr::start("tests/stubs/resp/template/smoke-isolation").await;
    get(&srv.path("/api/a")).await.expect_status_ok().expect_body_text_eq("/api/a");
    get(&srv.path("/api/b")).await.expect_status_ok().expect_body_text_eq("/api/b");
    get(&srv.path("/api/a")).await.expect_status_ok().expect_body_text_eq("/api/a");
    get(&srv.path("/api/b")).await.expect_status_ok().expect_body_text_eq("/api/b");
}
