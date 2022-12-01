use asserhttp::*;
use surf::get;

use crate::utils::*;

#[async_std::test]
#[stubr::mock("req/url-pattern/just-url.json")]
async fn should_map_request_url_pattern_with_just_url() {
    get(stubr.path("/api/pattern/abcd")).await.expect_status_ok();
    get(stubr.path("/api/pattern/1234")).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/url-pattern/just-one-query.json")]
async fn should_map_request_url_pattern_with_just_one_query() {
    get(stubr.path_query("/api/pattern", "one", "abcd")).await.expect_status_ok();
    get(stubr.path_query("/api/pattern", "one", "1234"))
        .await
        .expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/url-pattern/many-queries.json")]
async fn should_map_request_url_pattern_with_many_queries() {
    get(stubr.path_queries("/api/pattern", ("one", "abcd"), ("two", "abcd")))
        .await
        .expect_status_ok();
    get(stubr.path_queries("/api/pattern", ("one", "1234"), ("two", "abcd")))
        .await
        .expect_status_not_found();
    get(stubr.path_queries("/api/pattern", ("one", "abcd"), ("two", "1234")))
        .await
        .expect_status_not_found();
    get(stubr.path_queries("/api/pattern", ("one", "1234"), ("two", "1234")))
        .await
        .expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/url-pattern/all.json")]
async fn should_map_request_url_pattern_with_both_url_and_queries() {
    get(stubr.path_queries("/api/pattern/abcd", ("one", "abcd"), ("two", "abcd")))
        .await
        .expect_status_ok();
    get(stubr.path_queries("/api/pattern/1234", ("one", "abcd"), ("two", "abcd")))
        .await
        .expect_status_not_found();
    get(stubr.path_queries("/api/pattern/abcd", ("one", "1234"), ("two", "abcd")))
        .await
        .expect_status_not_found();
    get(stubr.path_queries("/api/pattern/abcd", ("one", "abcd"), ("two", "1234")))
        .await
        .expect_status_not_found();
    get(stubr.path_queries("/api/pattern/abcd", ("one", "1234"), ("two", "1234")))
        .await
        .expect_status_not_found();
    get(stubr.path_queries("/api/pattern/1234", ("one", "1234"), ("two", "1234")))
        .await
        .expect_status_not_found();
}
