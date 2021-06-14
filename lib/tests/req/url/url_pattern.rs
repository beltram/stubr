use asserhttp::*;
use surf::get;

use crate::utils::*;

#[async_std::test]
async fn should_map_request_url_pattern_with_just_url() {
    let srv = given("req/url-pattern/just-url");
    get(&srv.path("/api/pattern/abcd")).await.expect_status_ok();
    get(&srv.path("/api/pattern/1234")).await.expect_status_not_found();
}

#[async_std::test]
async fn should_map_request_url_pattern_with_just_one_query() {
    let srv = given("req/url-pattern/just-one-query");
    get(&srv.path_query("/api/pattern", "one", "abcd")).await.expect_status_ok();
    get(&srv.path_query("/api/pattern", "one", "1234")).await.expect_status_not_found();
}

#[async_std::test]
async fn should_map_request_url_pattern_with_many_queries() {
    let srv = given("req/url-pattern/many-queries");
    get(&srv.path_queries("/api/pattern", ("one", "abcd"), ("two", "abcd"))).await.expect_status_ok();
    get(&srv.path_queries("/api/pattern", ("one", "1234"), ("two", "abcd"))).await.expect_status_not_found();
    get(&srv.path_queries("/api/pattern", ("one", "abcd"), ("two", "1234"))).await.expect_status_not_found();
    get(&srv.path_queries("/api/pattern", ("one", "1234"), ("two", "1234"))).await.expect_status_not_found();
}

#[async_std::test]
async fn should_map_request_url_pattern_with_both_url_and_queries() {
    let srv = given("req/url-pattern/all");
    get(&srv.path_queries("/api/pattern/abcd", ("one", "abcd"), ("two", "abcd"))).await.expect_status_ok();
    get(&srv.path_queries("/api/pattern/1234", ("one", "abcd"), ("two", "abcd"))).await.expect_status_not_found();
    get(&srv.path_queries("/api/pattern/abcd", ("one", "1234"), ("two", "abcd"))).await.expect_status_not_found();
    get(&srv.path_queries("/api/pattern/abcd", ("one", "abcd"), ("two", "1234"))).await.expect_status_not_found();
    get(&srv.path_queries("/api/pattern/abcd", ("one", "1234"), ("two", "1234"))).await.expect_status_not_found();
    get(&srv.path_queries("/api/pattern/1234", ("one", "1234"), ("two", "1234"))).await.expect_status_not_found();
}
