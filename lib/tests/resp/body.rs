use asserhttp::*;
use serde_json::{Map, Value};
use serde_json::json;
use surf::get;

use crate::utils::*;

#[async_std::test]
async fn should_map_text_response_body() {
    let srv = given("resp/body/text");
    get(&srv.uri()).await
        .expect_status_ok()
        .expect_body_text_eq("Hello World !")
        .expect_content_type_text();
}

#[async_std::test]
async fn should_map_blank_text_response_body() {
    let srv = given("resp/body/text-blank");
    get(&srv.uri()).await
        .expect_status_ok()
        .expect_body_text_eq(" ")
        .expect_content_type_text();
}

#[async_std::test]
async fn should_map_empty_text_response_body() {
    let srv = given("resp/body/text-empty");
    get(&srv.uri()).await
        .expect_status_ok()
        .expect_body_absent()
        .expect_content_type_text();
}

#[async_std::test]
async fn should_map_json_response_body() {
    let srv = given("resp/body/json");
    let expected = json!({"name": "john", "age": 42, "candidate": true, "surnames": ["jdoe", "johnny"]});
    get(&srv.uri()).await
        .expect_status_ok()
        .expect_body_json_eq(expected)
        .expect_content_type_json();
}

#[async_std::test]
async fn should_map_empty_json_response_body() {
    let srv = given("resp/body/json-empty");
    get(&srv.uri()).await
        .expect_status_ok()
        .expect_body_json_eq(Value::Object(Map::default()))
        .expect_content_type_json();
}

#[async_std::test]
async fn from_file_should_map_from_json_file() {
    let srv = given("resp/body/body-file-json");
    get(&srv.uri()).await
        .expect_status_ok()
        .expect_body_json_eq(json!({"name": "jdoe", "age": 4}))
        .expect_content_type_json();
}

#[async_std::test]
async fn from_file_should_map_from_txt_file() {
    let srv = given("resp/body/body-file-txt");
    get(&srv.uri()).await
        .expect_status_ok()
        .expect_body_text_eq("jdoe,4")
        .expect_content_type_text();
}

#[async_std::test]
async fn from_file_should_fail_when_not_a_valid_path() {
    let srv = given("resp/body/body-file-not-path");
    get(&srv.uri()).await.expect_status_internal_server_error();
}

#[async_std::test]
async fn from_file_should_fail_when_file_does_not_exist() {
    let srv = given("resp/body/body-file-not-existing");
    get(&srv.uri()).await.expect_status_internal_server_error();
}

#[async_std::test]
async fn from_file_should_fail_when_invalid_json_in_file() {
    let srv = given("resp/body/body-file-invalid-json");
    get(&srv.uri()).await.expect_status_internal_server_error();
}
