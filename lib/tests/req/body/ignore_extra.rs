use serde_json::json;
use surf::post;

use crate::utils::*;

#[async_std::test]
async fn should_match_req_body_equal_to_json_ignoring_extra_elements() {
    let srv = given("req/body/eq/ignore/extra/on");
    post(&srv.uri()).body(json!({"name": "juste", "age": 42})).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_not_match_req_body_equal_to_json_ignoring_extra_elements_when_key_mismatches() {
    let srv = given("req/body/eq/ignore/extra/on");
    post(&srv.uri()).body(json!({"not-name": "juste", "age": 42})).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_not_match_req_body_equal_to_json_ignoring_extra_elements_when_value_mismatches() {
    let srv = given("req/body/eq/ignore/extra/on");
    post(&srv.uri()).body(json!({"name": "not-juste", "age": 42})).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_match_req_body_equal_to_json_not_ignoring_extra_elements() {
    let srv = given("req/body/eq/ignore/extra/off");
    post(&srv.uri()).body(json!({"name": "juste", "age": 42})).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn by_default_should_match_req_body_equal_to_json_not_ignoring_extra_elements() {
    let srv = given("req/body/eq/ignore/extra/default");
    post(&srv.uri()).body(json!({"name": "juste", "age": 42})).await.unwrap().assert_not_found();
}