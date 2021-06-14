use asserhttp::*;
use surf::get;

use crate::utils::*;

#[async_std::test]
async fn should_default_to_case_sensitive() {
    let srv = given("req/headers/equal/string");
    get(&srv.uri()).header("Content-Type", "Application/Json").await.expect_status_not_found();
}

#[async_std::test]
async fn should_support_case_insensitive() {
    let srv = given("req/headers/case/insensitive");
    get(&srv.uri()).header("Content-Type", "Application/Json").await.expect_status_ok();
    get(&srv.uri()).header("Content-Type", "application/json").await.expect_status_ok();
}

#[async_std::test]
async fn insensitive_should_fail_when_invalid_value() {
    let srv = given("req/headers/case/insensitive");
    get(&srv.uri()).header("Content-Type", "application/xml").await.expect_status_not_found();
}

#[async_std::test]
async fn insensitive_should_fail_when_invalid_key() {
    let srv = given("req/headers/case/insensitive");
    get(&srv.uri()).header("Not-Content-Type", "application/json").await.expect_status_not_found();
}

#[async_std::test]
async fn insensitive_should_fail_when_missing() {
    let srv = given("req/headers/case/insensitive");
    get(&srv.uri()).await.expect_status_not_found();
}

#[async_std::test]
async fn should_support_many_case_insensitive() {
    let srv = given("req/headers/case/insensitive-many");
    get(&srv.uri())
        .header("Content-Type", "Application/Json")
        .header("Accept", "Application/Json")
        .await
        .expect_status_ok();
    get(&srv.uri())
        .header("Content-Type", "application/json")
        .header("Accept", "Application/Json")
        .await
        .expect_status_ok();
    get(&srv.uri())
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .await
        .expect_status_ok();
}

#[async_std::test]
async fn should_fail_with_many_case_insensitive_string_value_when_one_of_does_not_match() {
    let srv = given("req/headers/case/insensitive-many");
    get(&srv.uri())
        .header("Content-Type", "application/xml")
        .header("Accept", "application/json")
        .await
        .expect_status_not_found();
    get(&srv.uri())
        .header("Content-Type", "application/json")
        .header("Accept", "application/xml")
        .await
        .expect_status_not_found();
    get(&srv.uri()).header("Content-Type", "application/json").await.expect_status_not_found();
    get(&srv.uri()).header("Accept", "application/json").await.expect_status_not_found();
}

#[async_std::test]
async fn should_support_explicit_case_sensitive() {
    let srv = given("req/headers/case/sensitive");
    get(&srv.uri()).header("Content-Type", "Application/Json").await.expect_status_not_found();
    get(&srv.uri()).header("Content-Type", "application/json").await.expect_status_ok();
}

#[async_std::test]
async fn sensitive_should_fail_when_invalid_value() {
    let srv = given("req/headers/case/sensitive");
    get(&srv.uri()).header("Content-Type", "application/xml").await.expect_status_not_found();
}

#[async_std::test]
async fn sensitive_should_fail_when_invalid_key() {
    let srv = given("req/headers/case/sensitive");
    get(&srv.uri()).header("Not-Content-Type", "application/json").await.expect_status_not_found();
}

#[async_std::test]
async fn sensitive_should_fail_when_missing() {
    let srv = given("req/headers/case/sensitive");
    get(&srv.uri()).await.expect_status_not_found();
}