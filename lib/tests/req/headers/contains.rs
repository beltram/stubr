use surf::get;

use crate::utils::*;

#[async_std::test]
async fn should_not_default_to_contains() {
    let srv = given("req/headers/equal/string");
    get(&srv.uri()).header("Content-Type", "json").await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_support_contains() {
    let srv = given("req/headers/contains/single");
    get(&srv.uri()).header("Content-Type", "application/json").await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_fail_when_does_not_contain() {
    let srv = given("req/headers/contains/single");
    get(&srv.uri()).header("Content-Type", "application/xml").await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_fail_when_invalid_key() {
    let srv = given("req/headers/contains/single");
    get(&srv.uri()).header("Not-Content-Type", "application/json").await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_fail_when_missing() {
    let srv = given("req/headers/contains/single");
    get(&srv.uri()).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_support_many_contains() {
    let srv = given("req/headers/contains/many");
    get(&srv.uri())
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .await.unwrap()
        .assert_ok();
}

#[async_std::test]
async fn should_fail_when_one_of_does_not_contains() {
    let srv = given("req/headers/contains/many");
    get(&srv.uri())
        .header("Content-Type", "application/xml")
        .header("Accept", "application/json")
        .await.unwrap()
        .assert_not_found();
    get(&srv.uri())
        .header("Content-Type", "application/json")
        .header("Accept", "application/xml")
        .await.unwrap()
        .assert_not_found();
    get(&srv.uri()).header("Content-Type", "application/json").await.unwrap().assert_not_found();
    get(&srv.uri()).header("Accept", "application/json").await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_support_contains_begin() {
    let srv = given("req/headers/contains/begin");
    get(&srv.uri()).header("Content-Type", "application/json").await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_support_contains_middle() {
    let srv = given("req/headers/contains/middle");
    get(&srv.uri()).header("Content-Type", "application/json").await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_support_contains_end() {
    let srv = given("req/headers/contains/end");
    get(&srv.uri()).header("Content-Type", "application/json").await.unwrap().assert_ok();
}