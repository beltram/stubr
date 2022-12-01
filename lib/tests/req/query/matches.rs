use asserhttp::*;
use surf::get;

use crate::utils::*;

#[async_std::test]
#[stubr::mock("req/query/matches/single.json")]
async fn should_map_request_when_query_matches() {
    get(stubr.query("age", "string")).await.expect_status_ok();
    get(stubr.query("age", "any")).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/query/matches/single.json")]
async fn should_fail_when_absent() {
    get(stubr.uri()).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/query/matches/single.json")]
async fn should_fail_when_regex_not_respected() {
    get(stubr.query("age", "1234")).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/query/matches/many.json")]
async fn should_map_request_when_many_query_matches() {
    get(stubr.queries(("age", "string"), ("city", "string")))
        .await
        .expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/query/matches/many.json")]
async fn should_fail_when_one_of_query_regex_not_respected() {
    get(stubr.queries(("age", "1234"), ("city", "string")))
        .await
        .expect_status_not_found();
    get(stubr.queries(("age", "string"), ("city", "1234")))
        .await
        .expect_status_not_found();
    get(stubr.queries(("age", "1234"), ("city", "1234")))
        .await
        .expect_status_not_found();
    get(stubr.query("age", "string")).await.expect_status_not_found();
    get(stubr.query("city", "string")).await.expect_status_not_found();
    get(stubr.uri()).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/query/matches/negative.json")]
async fn negative_should_map_request_when_query_matches() {
    get(stubr.query("age", "1234")).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/query/matches/negative.json")]
async fn negative_should_fail_when_absent() {
    get(stubr.uri()).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/query/matches/negative.json")]
async fn negative_should_fail_when_regex_not_respected() {
    get(stubr.query("age", "string")).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/query/matches/negative-many.json")]
async fn negative_should_map_request_when_many_query_matches() {
    get(stubr.queries(("age", "1234"), ("city", "1234"))).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/query/matches/negative-many.json")]
async fn negative_should_fail_when_one_of_query_regex_not_respected() {
    get(stubr.queries(("age", "string"), ("city", "1234")))
        .await
        .expect_status_not_found();
    get(stubr.queries(("age", "1234"), ("city", "string")))
        .await
        .expect_status_not_found();
    get(stubr.queries(("age", "string"), ("city", "string")))
        .await
        .expect_status_not_found();
    get(stubr.query("age", "1234")).await.expect_status_not_found();
    get(stubr.query("city", "1234")).await.expect_status_not_found();
    get(stubr.uri()).await.expect_status_not_found();
}
