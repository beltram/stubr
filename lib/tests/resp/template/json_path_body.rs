use serde_json::json;
use surf::post;

use crate::utils::*;

#[async_std::test]
async fn should_template_with_flat_request_body() {
    let srv = given("resp/template/body/json-path-simple");
    post(&srv.url()).body(json!({"name": "bob", "age": 42})).await.unwrap()
        .assert_ok()
        .assert_body_text("bob")
        .assert_content_type_text();
}

#[async_std::test]
async fn should_template_in_json_response_body() {
    let srv = given("resp/template/body/json-path-in-json-response-body");
    let body = json!({"firstname": "beltram", "lastname": "maldant"});
    post(&srv.url()).body(body.clone()).await.unwrap()
        .assert_ok()
        .assert_body_json(body)
        .assert_content_type_json();
}

#[async_std::test]
async fn should_template_with_complex_request_body() {
    let srv = given("resp/template/body/json-path-nested");
    post(&srv.url()).body(json!({"client": {"name": "bob", "age": 42}})).await.unwrap()
        .assert_ok()
        .assert_body_text("bob")
        .assert_content_type_text();
}

#[async_std::test]
async fn should_template_with_array_request_body() {
    let srv = given("resp/template/body/json-path-array");
    post(&srv.url()).body(json!({"names": ["alice", "bob"]})).await.unwrap()
        .assert_ok()
        .assert_body_text("alice")
        .assert_content_type_text();
}

#[async_std::test]
async fn should_not_template_when_key_absent_in_request_body() {
    let srv = given("resp/template/body/json-path-simple");
    post(&srv.url()).body(json!({"age": "43"})).await.unwrap()
        .assert_ok()
        .assert_body_text("")
        .assert_content_type_text();
}

#[async_std::test]
async fn should_not_template_when_request_body_absent() {
    let srv = given("resp/template/body/json-path-simple");
    post(&srv.url()).await.unwrap()
        .assert_ok()
        .assert_body_text("")
        .assert_content_type_text();
}