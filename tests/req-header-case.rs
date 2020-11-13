use surf::get;

use crate::utils::*;

mod utils;

#[async_std::test]
async fn should_default_to_case_sensitive() {
    let srv = given("req/headers/equal/string");
    get(&srv.uri()).set_header("Content-Type", "Application/Json").await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_support_case_insensitive() {
    let srv = given("req/headers/case/insensitive");
    get(&srv.uri()).set_header("Content-Type", "Application/Json").await.unwrap().assert_ok();
    get(&srv.uri()).set_header("Content-Type", "application/json").await.unwrap().assert_ok();
}

#[async_std::test]
async fn insensitive_should_fail_when_invalid_value() {
    let srv = given("req/headers/case/insensitive");
    get(&srv.uri()).set_header("Content-Type", "application/xml").await.unwrap().assert_not_found();
}

#[async_std::test]
async fn insensitive_should_fail_when_invalid_key() {
    let srv = given("req/headers/case/insensitive");
    get(&srv.uri()).set_header("Not-Content-Type", "application/json").await.unwrap().assert_not_found();
}

#[async_std::test]
async fn insensitive_should_fail_when_missing() {
    let srv = given("req/headers/case/insensitive");
    get(&srv.uri()).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_support_many_case_insensitive() {
    let srv = given("req/headers/case/insensitive-many");
    get(&srv.uri())
        .set_header("Content-Type", "Application/Json")
        .set_header("Accept", "Application/Json")
        .await.unwrap()
        .assert_ok();
    get(&srv.uri())
        .set_header("Content-Type", "application/json")
        .set_header("Accept", "Application/Json")
        .await.unwrap()
        .assert_ok();
    get(&srv.uri())
        .set_header("Content-Type", "application/json")
        .set_header("Accept", "application/json")
        .await.unwrap()
        .assert_ok();
}

#[async_std::test]
async fn should_fail_with_many_case_insensitive_string_value_when_one_of_does_not_match() {
    let srv = given("req/headers/case/insensitive-many");
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
async fn should_support_explicit_case_sensitive() {
    let srv = given("req/headers/case/sensitive");
    get(&srv.uri()).set_header("Content-Type", "Application/Json").await.unwrap().assert_not_found();
    get(&srv.uri()).set_header("Content-Type", "application/json").await.unwrap().assert_ok();
}

#[async_std::test]
async fn sensitive_should_fail_when_invalid_value() {
    let srv = given("req/headers/case/sensitive");
    get(&srv.uri()).set_header("Content-Type", "application/xml").await.unwrap().assert_not_found();
}

#[async_std::test]
async fn sensitive_should_fail_when_invalid_key() {
    let srv = given("req/headers/case/sensitive");
    get(&srv.uri()).set_header("Not-Content-Type", "application/json").await.unwrap().assert_not_found();
}

#[async_std::test]
async fn sensitive_should_fail_when_missing() {
    let srv = given("req/headers/case/sensitive");
    get(&srv.uri()).await.unwrap().assert_not_found();
}