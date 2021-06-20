use asserhttp::*;
use serde_json::json;
use surf::post;

#[async_std::test]
#[stubr::mock("resp/template/body/json-path-simple.json")]
async fn should_template_with_flat_request_body() {
    post(stubr.uri()).body(json!({"name": "bob", "age": 42})).await
        .expect_status_ok()
        .expect_body_text_eq("bob")
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/body/json-path-in-json-response-body.json")]
async fn should_template_in_json_response_body() {
    let body = json!({"firstname": "beltram", "lastname": "maldant"});
    post(stubr.uri()).body(body.clone()).await
        .expect_status_ok()
        .expect_body_json_eq(body)
        .expect_content_type_json();
}

#[async_std::test]
#[stubr::mock("resp/template/body/json-path-nested.json")]
async fn should_template_with_complex_request_body() {
    post(stubr.uri()).body(json!({"client": {"name": "bob", "age": 42}})).await
        .expect_status_ok()
        .expect_body_text_eq("bob")
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/body/json-path-array.json")]
async fn should_template_with_array_request_body() {
    post(stubr.uri()).body(json!({"names": ["alice", "bob"]})).await
        .expect_status_ok()
        .expect_body_text_eq("alice")
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/body/json-path-simple.json")]
async fn should_not_template_when_key_absent_in_request_body() {
    post(stubr.uri()).body(json!({"age": "43"})).await
        .expect_status_ok()
        .expect_body_absent()
        .expect_content_type_text();
}

#[async_std::test]
#[stubr::mock("resp/template/body/json-path-simple.json")]
async fn should_not_template_when_request_body_absent() {
    post(stubr.uri()).await
        .expect_status_ok()
        .expect_body_absent()
        .expect_content_type_text();
}