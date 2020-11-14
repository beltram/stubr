use serde::Deserialize;
use serde_json::{Map, Value};
use surf::get;

use crate::utils::*;

mod utils;

#[async_std::test]
async fn should_map_text_response_body() {
    let srv = given("resp/body/text");
    get(&srv.uri()).await.unwrap()
        .assert_ok()
        .assert_body_text("Hello World !")
        .assert_header("Content-Type", "text/plain");
}

#[async_std::test]
async fn should_map_blank_text_response_body() {
    let srv = given("resp/body/text-blank");
    get(&srv.uri()).await.unwrap()
        .assert_ok()
        .assert_body_text(" ")
        .assert_header("Content-Type", "text/plain");
}

#[async_std::test]
async fn should_map_empty_text_response_body() {
    let srv = given("resp/body/text-empty");
    get(&srv.uri()).await.unwrap()
        .assert_ok()
        .assert_body_empty()
        .assert_header("Content-Type", "text/plain");
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
struct JsonResp {
    name: String,
    age: u8,
    candidate: bool,
    surnames: Vec<String>,
}

#[async_std::test]
async fn should_map_json_response_body() {
    let srv = given("resp/body/json");
    let surnames = vec!["jdoe".to_string(), "johnny".to_string()];
    let expected = JsonResp { name: "john".to_string(), age: 42, candidate: true, surnames };
    get(&srv.uri()).await.unwrap()
        .assert_ok()
        .assert_body_json(expected)
        .assert_header("Content-Type", "application/json");
}

#[async_std::test]
async fn should_map_empty_json_response_body() {
    let srv = given("resp/body/json-empty");
    get(&srv.uri()).await.unwrap()
        .assert_ok()
        .assert_body_json(Value::Object(Map::default()))
        .assert_header("Content-Type", "application/json");
}
