use serde_json::json;
use surf::post;

use crate::utils::*;

mod utils;

#[async_std::test]
async fn should_map_req_body_equal_to_json() {
    let srv = given("req/body/eq-json");
    post(&srv.uri())
        .body(json!({"name": "bob"}))
        .await.unwrap()
        .assert_ok();
}

#[async_std::test]
async fn should_fail_when_req_body_key_not_equal_to_json() {
    let srv = given("req/body/eq-json");
    post(&srv.uri())
        .body(json!({"not-name": "bob"}))
        .await.unwrap()
        .assert_not_found();
}

#[async_std::test]
async fn should_fail_when_req_body_value_not_equal_to_json() {
    let srv = given("req/body/eq-json");
    post(&srv.uri())
        .body(json!({"name": "not-bob"}))
        .await.unwrap()
        .assert_not_found();
}