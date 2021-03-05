use surf::get;

use crate::utils::*;

#[async_std::test]
async fn should_default_to_case_sensitive() {
    let srv = given("req/headers/equal/string");
    get(&srv.url()).header("Content-Type", "Application/Json").await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_support_case_insensitive() {
    let srv = given("req/headers/case/insensitive");
    get(&srv.url()).header("Content-Type", "Application/Json").await.unwrap().assert_ok();
    get(&srv.url()).header("Content-Type", "application/json").await.unwrap().assert_ok();
}

#[async_std::test]
async fn insensitive_should_fail_when_invalid_value() {
    let srv = given("req/headers/case/insensitive");
    get(&srv.url()).header("Content-Type", "application/xml").await.unwrap().assert_not_found();
}

#[async_std::test]
async fn insensitive_should_fail_when_invalid_key() {
    let srv = given("req/headers/case/insensitive");
    get(&srv.url()).header("Not-Content-Type", "application/json").await.unwrap().assert_not_found();
}

#[async_std::test]
async fn insensitive_should_fail_when_missing() {
    let srv = given("req/headers/case/insensitive");
    get(&srv.url()).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_support_many_case_insensitive() {
    let srv = given("req/headers/case/insensitive-many");
    get(&srv.url())
        .header("Content-Type", "Application/Json")
        .header("Accept", "Application/Json")
        .await.unwrap()
        .assert_ok();
    get(&srv.url())
        .header("Content-Type", "application/json")
        .header("Accept", "Application/Json")
        .await.unwrap()
        .assert_ok();
    get(&srv.url())
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .await.unwrap()
        .assert_ok();
}

#[async_std::test]
async fn should_fail_with_many_case_insensitive_string_value_when_one_of_does_not_match() {
    let srv = given("req/headers/case/insensitive-many");
    get(&srv.url())
        .header("Content-Type", "application/xml")
        .header("Accept", "application/json")
        .await.unwrap()
        .assert_not_found();
    get(&srv.url())
        .header("Content-Type", "application/json")
        .header("Accept", "application/xml")
        .await.unwrap()
        .assert_not_found();
    get(&srv.url()).header("Content-Type", "application/json").await.unwrap().assert_not_found();
    get(&srv.url()).header("Accept", "application/json").await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_support_explicit_case_sensitive() {
    let srv = given("req/headers/case/sensitive");
    get(&srv.url()).header("Content-Type", "Application/Json").await.unwrap().assert_not_found();
    get(&srv.url()).header("Content-Type", "application/json").await.unwrap().assert_ok();
}

#[async_std::test]
async fn sensitive_should_fail_when_invalid_value() {
    let srv = given("req/headers/case/sensitive");
    get(&srv.url()).header("Content-Type", "application/xml").await.unwrap().assert_not_found();
}

#[async_std::test]
async fn sensitive_should_fail_when_invalid_key() {
    let srv = given("req/headers/case/sensitive");
    get(&srv.url()).header("Not-Content-Type", "application/json").await.unwrap().assert_not_found();
}

#[async_std::test]
async fn sensitive_should_fail_when_missing() {
    let srv = given("req/headers/case/sensitive");
    get(&srv.url()).await.unwrap().assert_not_found();
}