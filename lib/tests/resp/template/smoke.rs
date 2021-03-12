use serde_json::json;
use surf::get;

use crate::utils::*;

#[async_std::test]
async fn should_template_in_body() {
    let srv = given("resp/template/smoke/in-body");
    get(&srv.path("/api/path")).await.unwrap()
        .assert_ok()
        .assert_body_text("/api/path")
        .assert_content_type_text();
}

#[async_std::test]
async fn should_not_template_in_body_when_no_placeholder() {
    let srv = given("resp/template/smoke/in-body-unused");
    get(&srv.path("/api/path")).await.unwrap()
        .assert_ok()
        .assert_body_text("any")
        .assert_content_type_text();
}

#[async_std::test]
async fn should_template_in_json_body() {
    let srv = given("resp/template/smoke/in-json-body");
    get(&srv.path("/api/path")).await.unwrap()
        .assert_ok()
        .assert_body_json(json!({"path": "/api/path"}))
        .assert_content_type_json();
}

#[async_std::test]
async fn should_not_template_in_json_body_when_no_placeholder() {
    let srv = given("resp/template/smoke/in-json-body-unused");
    get(&srv.path("/api/path")).await.unwrap()
        .assert_ok()
        .assert_body_json(json!({"path": "any"}))
        .assert_content_type_json();
}

#[async_std::test]
async fn should_template_in_json_file_body() {
    let srv = given("resp/template/smoke/in-body-json-file");
    get(&srv.path("/api/path")).await.unwrap()
        .assert_ok()
        .assert_body_json(json!({"path": "/api/path"}))
        .assert_content_type_json();
}

#[async_std::test]
async fn should_not_template_in_json_file_body_when_no_placeholder() {
    let srv = given("resp/template/smoke/in-body-json-file-unused");
    get(&srv.path("/api/path")).await.unwrap()
        .assert_ok()
        .assert_body_json(json!({"path": "any"}))
        .assert_content_type_json();
}

#[async_std::test]
async fn should_template_in_text_file_body() {
    let srv = given("resp/template/smoke/in-body-text-file");
    get(&srv.path("/api/path")).await.unwrap()
        .assert_ok()
        .assert_body_text("Lorem ipsum /api/path dolor sit amet")
        .assert_content_type_text();
}

#[async_std::test]
async fn should_not_template_in_text_file_body_when_no_placeholder() {
    let srv = given("resp/template/smoke/in-body-text-file-unused");
    get(&srv.path("/api/path")).await.unwrap()
        .assert_ok()
        .assert_body_text("Lorem ipsum any dolor sit amet")
        .assert_content_type_text();
}

#[async_std::test]
async fn should_template_in_response_headers() {
    let srv = given("resp/template/smoke/in-header");
    get(&srv.path("/api/path")).await.unwrap()
        .assert_ok()
        .assert_header("x-req-path", "/api/path");
}

#[async_std::test]
async fn should_template_in_many_response_headers() {
    let srv = given("resp/template/smoke/in-many-header");
    get(&srv.path("/api/path")).await.unwrap()
        .assert_ok()
        .assert_header("x-1", "/api/path")
        .assert_header("x-2", "GET");
}

#[async_std::test]
async fn should_not_template_in_response_headers_when_no_placeholder() {
    let srv = given("resp/template/smoke/in-header-unused");
    get(&srv.path("/api/path")).await.unwrap()
        .assert_ok()
        .assert_header("x-req-path", "/the/path");
}

#[async_std::test]
async fn stubs_should_be_isolated() {
    let srv = Stubr::start("tests/stubs/resp/template/smoke-isolation").await;
    get(&srv.path("/api/a")).await.unwrap().assert_ok().assert_body_text("/api/a");
    get(&srv.path("/api/b")).await.unwrap().assert_ok().assert_body_text("/api/b");
    get(&srv.path("/api/a")).await.unwrap().assert_ok().assert_body_text("/api/a");
    get(&srv.path("/api/b")).await.unwrap().assert_ok().assert_body_text("/api/b");
}
