use asserhttp::*;
use serde_json::json;
use surf::post;

use crate::utils::*;

#[async_std::test]
async fn should_match_req_body_equal_to_json_ignoring_array_order() {
    let srv = given("req/body/eq/ignore/order/on");
    post(&srv.uri()).body(json!({"names": ["john", "doe"]})).await.expect_status_ok();
}

#[async_std::test]
async fn should_not_match_req_body_equal_to_json_ignoring_array_order_when_key_mismatches() {
    let srv = given("req/body/eq/ignore/order/on");
    post(&srv.uri()).body(json!({"not-names": ["john", "doe"]})).await.expect_status_not_found();
}

#[async_std::test]
async fn should_not_match_req_body_equal_to_json_ignoring_array_order_when_items_invalid() {
    let srv = given("req/body/eq/ignore/order/on");
    post(&srv.uri()).body(json!({"names": ["john"]})).await.expect_status_not_found();
    post(&srv.uri()).body(json!({"names": ["doe"]})).await.expect_status_not_found();
    post(&srv.uri()).body(json!({"names": []})).await.expect_status_not_found();
    post(&srv.uri()).body(json!({"names": ["john", "doe", "alfred"]})).await.expect_status_not_found();
}

#[async_std::test]
async fn should_match_req_body_equal_to_json_not_ignoring_array_order() {
    let srv = given("req/body/eq/ignore/order/off");
    post(&srv.uri()).body(json!({"names": ["john", "doe"]})).await.expect_status_ok();
    post(&srv.uri()).body(json!({"names": ["doe", "john"]})).await.expect_status_not_found();
}

#[async_std::test]
async fn by_default_should_match_req_body_equal_to_json_not_ignoring_array_order() {
    let srv = given("req/body/eq/ignore/order/default");
    post(&srv.uri()).body(json!({"names": ["john", "doe"]})).await.expect_status_ok();
    post(&srv.uri()).body(json!({"names": ["doe", "john"]})).await.expect_status_not_found();
}

#[async_std::test]
async fn ignoring_array_order_should_not_allow_extra_elements() {
    let srv = given("req/body/eq/ignore/order/on");
    post(&srv.uri()).body(json!({"names": ["john", "doe"], "age": 42})).await.expect_status_not_found();
}

#[async_std::test]
async fn should_match_req_body_equal_to_json_ignoring_array_order_for_root_array() {
    let srv = given("req/body/eq/ignore/order/root-array");
    post(&srv.uri()).body(json!(["john", "doe"])).await.expect_status_ok();
    post(&srv.uri()).body(json!(["doe", "john"])).await.expect_status_ok();
}
