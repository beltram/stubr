use asserhttp::*;
use surf::get;

use crate::utils::*;

#[async_std::test]
#[stubr::mock("req/query/equal/string.json")]
async fn should_not_default_to_contains() {
    get(stubr.query("age", "u")).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/query/contains/single.json")]
async fn should_support_contains() {
    get(stubr.query("age", "young")).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/query/contains/single.json")]
async fn should_fail_when_does_not_contain() {
    get(stubr.query("age", "old")).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/query/contains/single.json")]
async fn should_fail_when_invalid_key() {
    get(stubr.query("not-age", "young")).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/query/contains/single.json")]
async fn should_fail_when_missing() {
    get(stubr.uri()).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/query/contains/many.json")]
async fn should_support_many_contains() {
    get(stubr.queries(("age", "young"), ("city", "paris"))).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/query/contains/many.json")]
async fn should_fail_when_one_of_does_not_contains() {
    get(stubr.queries(("age", "old"), ("city", "paris"))).await.expect_status_not_found();
    get(stubr.queries(("age", "young"), ("city", "lyon"))).await.expect_status_not_found();
    get(stubr.query("age", "young")).await.expect_status_not_found();
    get(stubr.query("city", "paris")).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/query/contains/begin.json")]
async fn should_support_contains_begin() {
    get(stubr.query("age", "young")).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/query/contains/middle.json")]
async fn should_support_contains_middle() {
    get(stubr.query("age", "young")).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/query/contains/end.json")]
async fn should_support_contains_end() {
    get(stubr.query("age", "young")).await.expect_status_ok();
}