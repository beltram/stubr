use serde_json::json;
use surf::post;

use crate::utils::*;

#[async_std::test]
async fn should_map_req_body_equal_to_json_string() {
    let srv = given("req/body/eq/string");
    post(&srv.url()).body(json!({"name": "bob"})).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_fail_when_req_body_key_not_equal_to_json_string() {
    let srv = given("req/body/eq/string");
    post(&srv.url()).body(json!({"notName": "bob"})).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_fail_when_req_body_value_not_equal_to_json_string() {
    let srv = given("req/body/eq/string");
    post(&srv.url()).body(json!({"name": "not-bob"})).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_map_req_body_equal_to_json_bool() {
    let srv = given("req/body/eq/bool");
    post(&srv.url()).body(json!({"isGood": true})).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_fail_when_req_body_key_not_equal_to_json_bool() {
    let srv = given("req/body/eq/bool");
    post(&srv.url()).body(json!({"isNotGood": true})).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_fail_when_req_body_value_not_equal_to_json_bool() {
    let srv = given("req/body/eq/bool");
    post(&srv.url()).body(json!({"isGood": false})).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_map_req_body_equal_to_json_unsigned_number() {
    let srv = given("req/body/eq/unsigned-number");
    post(&srv.url()).body(json!({"age": 42})).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_fail_when_req_body_key_not_equal_to_json_unsigned_number() {
    let srv = given("req/body/eq/unsigned-number");
    post(&srv.url()).body(json!({"notAge": 42})).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_fail_when_req_body_value_not_equal_to_json_unsigned_number() {
    let srv = given("req/body/eq/unsigned-number");
    post(&srv.url()).body(json!({"age": 43})).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_map_req_body_equal_to_json_signed_number() {
    let srv = given("req/body/eq/signed-number");
    post(&srv.url()).body(json!({"age": -42})).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_fail_when_req_body_key_not_equal_to_json_signed_number() {
    let srv = given("req/body/eq/signed-number");
    post(&srv.url()).body(json!({"notAge": -42})).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_fail_when_req_body_value_not_equal_to_json_signed_number() {
    let srv = given("req/body/eq/signed-number");
    post(&srv.url()).body(json!({"age": -43})).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_map_req_body_equal_to_float() {
    let srv = given("req/body/eq/float");
    post(&srv.url()).body(json!({"pi": 3.14})).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_fail_when_req_body_key_not_equal_to_float() {
    let srv = given("req/body/eq/float");
    post(&srv.url()).body(json!({"notPi": 3.14})).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_fail_when_req_body_value_not_equal_to_float() {
    let srv = given("req/body/eq/float");
    post(&srv.url()).body(json!({"pi": 3.15})).await.unwrap().assert_not_found();
    post(&srv.url()).body(json!({"pi": 3.13})).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_map_req_body_equal_to_null() {
    let srv = given("req/body/eq/null");
    post(&srv.url()).body(json!({"maybe": null})).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_fail_when_req_body_key_not_equal_to_null() {
    let srv = given("req/body/eq/null");
    post(&srv.url()).body(json!({"sure": null})).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_fail_when_req_body_value_not_equal_to_null() {
    let srv = given("req/body/eq/null");
    post(&srv.url()).body(json!({"maybe": "some"})).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_map_req_body_equal_to_obj() {
    let srv = given("req/body/eq/obj");
    post(&srv.url()).body(json!({"user": {"name": "jdoe"}})).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_fail_when_req_body_key_not_equal_to_obj() {
    let srv = given("req/body/eq/obj");
    post(&srv.url()).body(json!({"notUser": {"name": "jdoe"}})).await.unwrap().assert_not_found();
    post(&srv.url()).body(json!({"user": {"NotName": "jdoe"}})).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_fail_when_req_body_value_not_equal_to_obj() {
    let srv = given("req/body/eq/obj");
    post(&srv.url()).body(json!({"user": {"name": "alice"}})).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_map_req_body_equal_to_array() {
    let srv = given("req/body/eq/array");
    post(&srv.url()).body(json!({"names": ["alice", "bob"]})).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_fail_when_req_body_key_not_equal_to_array() {
    let srv = given("req/body/eq/array");
    post(&srv.url()).body(json!({"notNames": ["alice", "bob"]})).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_fail_when_req_body_value_not_equal_to_array() {
    let srv = given("req/body/eq/array");
    post(&srv.url()).body(json!({"names": ["a", "bob"]})).await.unwrap().assert_not_found();
    post(&srv.url()).body(json!({"names": ["alice", "b"]})).await.unwrap().assert_not_found();
    post(&srv.url()).body(json!({"names": ["alice"]})).await.unwrap().assert_not_found();
    post(&srv.url()).body(json!({"names": ["bob"]})).await.unwrap().assert_not_found();
    post(&srv.url()).body(json!({"names": []})).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_match_req_body_equal_to_base64() {
    let srv = given("req/body/eq/binary");
    post(&srv.url()).body(vec![1, 2, 3]).await.unwrap().assert_ok();
    post(&srv.url()).body(vec![3, 2, 1]).await.unwrap().assert_not_found();
}