use surf::get;

use crate::utils::*;

mod utils;

#[async_std::test]
async fn should_not_default_to_contains() {
    let srv = given("req/query/equal/string");
    get(&srv.query("age", "u")).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_support_contains() {
    let srv = given("req/query/contains/single");
    get(&srv.query("age", "young")).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_fail_when_does_not_contain() {
    let srv = given("req/query/contains/single");
    get(&srv.query("age", "old")).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_fail_when_invalid_key() {
    let srv = given("req/query/contains/single");
    get(&srv.query("not-age", "young")).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_fail_when_missing() {
    let srv = given("req/query/contains/single");
    get(&srv.uri()).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_support_many_contains() {
    let srv = given("req/query/contains/many");
    get(&srv.queries(("age", "young"), ("city", "paris"))).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_fail_when_one_of_does_not_contains() {
    let srv = given("req/query/contains/many");
    get(&srv.queries(("age", "old"), ("city", "paris"))).await.unwrap().assert_not_found();
    get(&srv.queries(("age", "young"), ("city", "lyon"))).await.unwrap().assert_not_found();
    get(&srv.query("age", "young")).await.unwrap().assert_not_found();
    get(&srv.query("city", "paris")).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_support_contains_begin() {
    let srv = given("req/query/contains/begin");
    get(&srv.query("age", "young")).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_support_contains_middle() {
    let srv = given("req/query/contains/middle");
    get(&srv.query("age", "young")).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_support_contains_end() {
    let srv = given("req/query/contains/end");
    get(&srv.query("age", "young")).await.unwrap().assert_ok();
}