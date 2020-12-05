use serde_json::json;
use surf::post;

use crate::utils::*;

mod utils;

#[async_std::test]
async fn should_map_req_body_when_one_field_matches() {
    let srv = given("req/body/json-path/single");
    post(&srv.uri()).body(json!({"name": "bob"})).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_fail_when_none_matches() {
    let srv = given("req/body/json-path/single");
    post(&srv.uri()).body(json!({"notName": "bob"})).await.unwrap().assert_not_found();
    post(&srv.uri()).body(json!({})).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_map_req_body_when_many_fields_match() {
    let srv = given("req/body/json-path/many");
    post(&srv.uri()).body(json!({"name": "bob", "age": 42})).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_fail_when_one_of_many_does_not_match() {
    let srv = given("req/body/json-path/many");
    post(&srv.uri()).body(json!({"notName": "bob", "age": 42})).await.unwrap().assert_not_found();
    post(&srv.uri()).body(json!({"name": "bob", "notAge": 42})).await.unwrap().assert_not_found();
    post(&srv.uri()).body(json!({"name": "bob"})).await.unwrap().assert_not_found();
    post(&srv.uri()).body(json!({"age": 42})).await.unwrap().assert_not_found();
    post(&srv.uri()).body(json!({})).await.unwrap().assert_not_found();
}
