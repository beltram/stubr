use serde_json::json;
use surf::post;

use crate::utils::*;

#[async_std::test]
async fn should_map_req_body_when_one_field_matches() {
    let srv = given("req/body/json-path/single");
    post(&srv.url()).body(json!({"name": "bob"})).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_fail_when_none_matches() {
    let srv = given("req/body/json-path/single");
    post(&srv.url()).body(json!({"notName": "bob"})).await.unwrap().assert_not_found();
    post(&srv.url()).body(json!({})).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_map_req_body_when_many_fields_match() {
    let srv = given("req/body/json-path/many");
    post(&srv.url()).body(json!({"name": "bob", "age": 42})).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_fail_when_one_of_many_does_not_match() {
    let srv = given("req/body/json-path/many");
    post(&srv.url()).body(json!({"notName": "bob", "age": 42})).await.unwrap().assert_not_found();
    post(&srv.url()).body(json!({"name": "bob", "notAge": 42})).await.unwrap().assert_not_found();
    post(&srv.url()).body(json!({"name": "bob"})).await.unwrap().assert_not_found();
    post(&srv.url()).body(json!({"age": 42})).await.unwrap().assert_not_found();
    post(&srv.url()).body(json!({})).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn can_be_combined_with_eq() {
    let srv = given("req/body/json-path/plus-eq");
    post(&srv.url()).body(json!({"person": { "name": "bob" }})).await.unwrap().assert_ok();
    post(&srv.url()).body(json!({"person": { "name": "bob" }, "person": { "name": "bob" }})).await.unwrap().assert_ok();
    post(&srv.url()).body(json!({"person": { "name": "bob" }, "notPerson": { "name": "bob" }})).await.unwrap().assert_ok();
    post(&srv.url()).body(json!({"person": { "name": "bob" }, "person": { "name": "alice" }})).await.unwrap().assert_not_found();
    post(&srv.url()).body(json!({"person": { "name": "alice" }})).await.unwrap().assert_not_found();
    post(&srv.url()).body(json!({"person": { "notName": "bob" }})).await.unwrap().assert_not_found();
    post(&srv.url()).body(json!({"notPerson": { "name": "bob" }})).await.unwrap().assert_not_found();
    post(&srv.url()).body(json!({"person": { }})).await.unwrap().assert_not_found();
    post(&srv.url()).body(json!({})).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn can_be_combined_with_contains() {
    let srv = given("req/body/json-path/plus-contains");
    post(&srv.url()).body(json!({"name": "bob"})).await.unwrap().assert_ok();
    post(&srv.url()).body(json!({"name": "alice"})).await.unwrap().assert_not_found();
    post(&srv.url()).body(json!({"notName": "bob"})).await.unwrap().assert_not_found();
    post(&srv.url()).body(json!({"name": ""})).await.unwrap().assert_not_found();
    post(&srv.url()).body(json!({})).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_map_req_body_by_json_path_eq() {
    let srv = given("req/body/json-path/eq");
    post(&srv.url()).body(json!({"consoles": [ { "id": "xbox" } ]})).await.unwrap().assert_ok();
    post(&srv.url()).body(json!({"consoles": [ { "id": "xbox" }, { "id": "playstation" } ]})).await.unwrap().assert_ok();
    post(&srv.url()).body(json!({"consoles": [ { "id": "playstation" }, { "id": "xbox" } ]})).await.unwrap().assert_ok();
    post(&srv.url()).body(json!({"consoles": [ { "id": "playstation" }, { "id": "xbox" }, { "id": "switch" } ]})).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_fail_when_json_path_not_equals() {
    let srv = given("req/body/json-path/eq");
    post(&srv.url()).body(json!({"consoles": [ { "id": "playstation" } ]})).await.unwrap().assert_not_found();
    post(&srv.url()).body(json!({"consoles": [ {} ]})).await.unwrap().assert_not_found();
    post(&srv.url()).body(json!({"consoles": []})).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_map_req_body_by_json_path_greater_than() {
    let srv = given("req/body/json-path/gt");
    post(&srv.url()).body(json!({"consoles": [ { "price": 201 } ]})).await.unwrap().assert_ok();
    post(&srv.url()).body(json!({"consoles": [ { "price": 200 } ]})).await.unwrap().assert_not_found();
    post(&srv.url()).body(json!({"consoles": [ { "price": 199 } ]})).await.unwrap().assert_not_found();
    post(&srv.url()).body(json!({"consoles": [ { "price": 201 }, { "price": 199 } ]})).await.unwrap().assert_ok();
    post(&srv.url()).body(json!({"consoles": [ { "price": 199 }, { "price": 199 } ]})).await.unwrap().assert_not_found();
    post(&srv.url()).body(json!({"consoles": [ { } ]})).await.unwrap().assert_not_found();
    post(&srv.url()).body(json!({"consoles": [ ]})).await.unwrap().assert_not_found();
}