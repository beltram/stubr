use asserhttp::*;
use surf::get;

use crate::utils::*;

#[async_std::test]
async fn should_map_request_when_query_matches() {
    let srv = given("req/query/matches/single");
    get(&srv.query("age", "string")).await.expect_status_ok();
    get(&srv.query("age", "any")).await.expect_status_ok();
}

#[async_std::test]
async fn should_fail_when_absent() {
    let srv = given("req/query/matches/single");
    get(&srv.uri()).await.expect_status_not_found();
}

#[async_std::test]
async fn should_fail_when_regex_not_respected() {
    let srv = given("req/query/matches/single");
    get(&srv.query("age", "1234")).await.expect_status_not_found();
}

#[async_std::test]
async fn should_map_request_when_many_query_matches() {
    let srv = given("req/query/matches/many");
    get(&srv.queries(("age", "string"), ("city", "string")))
        .await
        .expect_status_ok();
}

#[async_std::test]
async fn should_fail_when_one_of_query_regex_not_respected() {
    let srv = given("req/query/matches/many");
    get(&srv.queries(("age", "1234"), ("city", "string"))).await.expect_status_not_found();
    get(&srv.queries(("age", "string"), ("city", "1234"))).await.expect_status_not_found();
    get(&srv.queries(("age", "1234"), ("city", "1234"))).await.expect_status_not_found();
    get(&srv.query("age", "string")).await.expect_status_not_found();
    get(&srv.query("city", "string")).await.expect_status_not_found();
    get(&srv.uri()).await.expect_status_not_found();
}

#[async_std::test]
async fn negative_should_map_request_when_query_matches() {
    let srv = given("req/query/matches/negative");
    get(&srv.query("age", "1234")).await.expect_status_ok();
}

#[async_std::test]
async fn negative_should_fail_when_absent() {
    let srv = given("req/query/matches/negative");
    get(&srv.uri()).await.expect_status_not_found();
}

#[async_std::test]
async fn negative_should_fail_when_regex_not_respected() {
    let srv = given("req/query/matches/negative");
    get(&srv.query("age", "string")).await.expect_status_not_found();
}

#[async_std::test]
async fn negative_should_map_request_when_many_query_matches() {
    let srv = given("req/query/matches/negative-many");
    get(&srv.queries(("age", "1234"), ("city", "1234")))
        .await
        .expect_status_ok();
}

#[async_std::test]
async fn negative_should_fail_when_one_of_query_regex_not_respected() {
    let srv = given("req/query/matches/negative-many");
    get(&srv.queries(("age", "string"), ("city", "1234"))).await.expect_status_not_found();
    get(&srv.queries(("age", "1234"), ("city", "string"))).await.expect_status_not_found();
    get(&srv.queries(("age", "string"), ("city", "string"))).await.expect_status_not_found();
    get(&srv.query("age", "1234")).await.expect_status_not_found();
    get(&srv.query("city", "1234")).await.expect_status_not_found();
    get(&srv.uri()).await.expect_status_not_found();
}
