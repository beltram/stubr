use serde_json::{Map, Value};
use serde_json::json;
use surf::get;

use crate::utils::*;

#[async_std::test]
async fn should_map_text_response_body() {
    let srv = given("resp/body/text");
    get(&srv.uri()).await.unwrap()
        .assert_ok()
        .assert_body_text("Hello World !")
        .assert_content_type_text();
}

#[async_std::test]
async fn should_map_blank_text_response_body() {
    let srv = given("resp/body/text-blank");
    get(&srv.uri()).await.unwrap()
        .assert_ok()
        .assert_body_text(" ")
        .assert_content_type_text();
}

#[async_std::test]
async fn should_map_empty_text_response_body() {
    let srv = given("resp/body/text-empty");
    get(&srv.uri()).await.unwrap()
        .assert_ok()
        .assert_body_empty()
        .assert_content_type_text();
}

#[async_std::test]
async fn should_map_json_response_body() {
    let srv = given("resp/body/json");
    let expected = json!({"name": "john", "age": 42, "candidate": true, "surnames": ["jdoe", "johnny"]});
    get(&srv.uri()).await.unwrap()
        .assert_ok()
        .assert_body_json(expected)
        .assert_content_type_json();
}

#[async_std::test]
async fn should_map_empty_json_response_body() {
    let srv = given("resp/body/json-empty");
    get(&srv.uri()).await.unwrap()
        .assert_ok()
        .assert_body_json(Value::Object(Map::default()))
        .assert_content_type_json();
}

#[async_std::test]
async fn from_file_should_map_from_json_file() {
    let srv = given("resp/body/body-file-json");
    get(&srv.uri()).await.unwrap()
        .assert_ok()
        .assert_body_json(json!({"name": "jdoe", "age": 4}))
        .assert_content_type_json();
}

#[async_std::test]
async fn from_file_should_map_from_txt_file() {
    let srv = given("resp/body/body-file-txt");
    get(&srv.uri()).await.unwrap()
        .assert_ok()
        .assert_body_text("jdoe,4")
        .assert_content_type_text();
}

#[async_std::test]
async fn from_file_should_fail_when_not_a_valid_path() {
    let srv = given("resp/body/body-file-not-path");
    get(&srv.uri()).await.unwrap().assert_error();
}

#[async_std::test]
async fn from_file_should_fail_when_file_does_not_exist() {
    let srv = given("resp/body/body-file-not-existing");
    get(&srv.uri()).await.unwrap().assert_error();
}

#[async_std::test]
async fn from_file_should_fail_when_invalid_json_in_file() {
    let srv = given("resp/body/body-file-invalid-json");
    get(&srv.uri()).await.unwrap().assert_error();
}
