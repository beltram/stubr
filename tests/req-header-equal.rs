use surf::get;

use crate::utils::*;

mod utils;

#[async_std::test]
async fn should_map_request_exact_string_value() {
    let srv = given("req/headers/equal/string");
    get(&srv.uri()).set_header("Content-Type", "application/json").await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_fail_when_incorrect_string_value() {
    let srv = given("req/headers/equal/string");
    get(&srv.uri()).set_header("Content-Type", "application/xml").await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_fail_when_invalid_key() {
    let srv = given("req/headers/equal/string");
    get(&srv.uri()).set_header("Not-Content-Type", "application/json").await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_fail_when_missing() {
    let srv = given("req/headers/equal/string");
    get(&srv.uri()).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_map_request_many_exact_string_value() {
    let srv = given("req/headers/equal/string-many");
    get(&srv.uri())
        .set_header("Content-Type", "application/json")
        .set_header("Accept", "application/json")
        .await.unwrap()
        .assert_ok();
}

#[async_std::test]
async fn should_fail_with_many_exact_string_value_when_one_of_does_not_match() {
    let srv = given("req/headers/equal/string-many");
    get(&srv.uri())
        .set_header("Content-Type", "application/xml")
        .set_header("Accept", "application/json")
        .await.unwrap()
        .assert_not_found();
    get(&srv.uri())
        .set_header("Content-Type", "application/json")
        .set_header("Accept", "application/xml")
        .await.unwrap()
        .assert_not_found();
    get(&srv.uri()).set_header("Content-Type", "application/json").await.unwrap().assert_not_found();
    get(&srv.uri()).set_header("Accept", "application/json").await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_map_request_exact_int_value() {
    let srv = given("req/headers/equal/int");
    get(&srv.uri()).set_header("Content-Type", "42").await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_fail_when_incorrect_int_value() {
    let srv = given("req/headers/equal/int");
    get(&srv.uri()).set_header("Content-Type", "43").await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_fail_when_not_an_int_value() {
    let srv = given("req/headers/equal/int");
    get(&srv.uri()).set_header("Content-Type", "application/json").await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_map_request_exact_bool_value() {
    let srv = given("req/headers/equal/bool");
    get(&srv.uri()).set_header("Content-Type", "true").await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_fail_when_incorrect_bool_value() {
    let srv = given("req/headers/equal/bool");
    get(&srv.uri()).set_header("Content-Type", "false").await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_fail_when_not_an_bool_value() {
    let srv = given("req/headers/equal/bool");
    get(&srv.uri()).set_header("Content-Type", "application/json").await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_map_request_many_exact_string_and_int_value() {
    let srv = given("req/headers/equal/string-int");
    get(&srv.uri())
        .set_header("Content-Type", "application/json")
        .set_header("Accept", "true")
        .await.unwrap()
        .assert_ok();
}