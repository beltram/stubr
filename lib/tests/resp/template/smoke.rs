use asserhttp::*;
use serde_json::json;
use surf::get;

#[async_std::test]
#[stubr::mock("resp/template/smoke/in-body.json")]
async fn should_template_in_body() {
    get(stubr.path("/api/path"))
        .await
        .expect_status_ok()
        .expect_body_text_eq("/api/path")
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/smoke/in-body-unused.json")]
async fn should_not_template_in_body_when_no_placeholder() {
    get(stubr.path("/api/path"))
        .await
        .expect_status_ok()
        .expect_body_text_eq("any")
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/smoke/in-json-body.json")]
async fn should_template_in_json_body() {
    get(stubr.path("/api/path"))
        .await
        .expect_status_ok()
        .expect_body_json_eq(json!({"path": "/api/path"}))
        .expect_content_type_json();
}

#[async_std::test]
#[stubr::mock("resp/template/smoke/in-json-body-unused.json")]
async fn should_not_template_in_json_body_when_no_placeholder() {
    get(stubr.path("/api/path"))
        .await
        .expect_status_ok()
        .expect_body_json_eq(json!({"path": "any"}))
        .expect_content_type_json();
}

#[async_std::test]
#[stubr::mock("resp/template/smoke/in-body-json-file.json")]
async fn should_template_in_json_file_body() {
    get(stubr.path("/api/path"))
        .await
        .expect_status_ok()
        .expect_body_json_eq(json!({"path": "/api/path"}))
        .expect_content_type_json();
}

#[async_std::test]
#[stubr::mock("resp/template/smoke/in-body-json-file-unused.json")]
async fn should_not_template_in_json_file_body_when_no_placeholder() {
    get(stubr.path("/api/path"))
        .await
        .expect_status_ok()
        .expect_body_json_eq(json!({"path": "any"}))
        .expect_content_type_json();
}

#[async_std::test]
#[stubr::mock("resp/template/smoke/in-body-text-file.json")]
async fn should_template_in_text_file_body() {
    get(stubr.path("/api/path"))
        .await
        .expect_status_ok()
        .expect_body_text_eq("Lorem ipsum /api/path dolor sit amet")
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/smoke/in-body-text-file-unused.json")]
async fn should_not_template_in_text_file_body_when_no_placeholder() {
    get(stubr.path("/api/path"))
        .await
        .expect_status_ok()
        .expect_body_text_eq("Lorem ipsum any dolor sit amet")
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/smoke/in-header.json")]
async fn should_template_in_response_headers() {
    get(stubr.path("/api/path"))
        .await
        .expect_status_ok()
        .expect_header("x-req-path", "/api/path");
}

#[async_std::test]
#[stubr::mock("resp/template/smoke/in-many-header.json")]
async fn should_template_in_many_response_headers() {
    get(stubr.path("/api/path"))
        .await
        .expect_status_ok()
        .expect_header("x-1", "/api/path")
        .expect_header("x-2", "GET");
}

#[async_std::test]
#[stubr::mock("resp/template/smoke/in-header-unused.json")]
async fn should_not_template_in_response_headers_when_no_placeholder() {
    get(stubr.path("/api/path"))
        .await
        .expect_status_ok()
        .expect_header("x-req-path", "/the/path");
}

#[async_std::test]
#[stubr::mock("resp/template/smoke-isolation")]
async fn stubs_should_be_isolated() {
    get(stubr.path("/api/a")).await.expect_status_ok().expect_body_text_eq("/api/a");
    get(stubr.path("/api/b")).await.expect_status_ok().expect_body_text_eq("/api/b");
    get(stubr.path("/api/a")).await.expect_status_ok().expect_body_text_eq("/api/a");
    get(stubr.path("/api/b")).await.expect_status_ok().expect_body_text_eq("/api/b");
}
