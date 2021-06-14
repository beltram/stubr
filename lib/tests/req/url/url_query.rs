use asserhttp::*;
use surf::get;

use crate::utils::*;

#[async_std::test]
async fn should_map_request_url_and_query() {
    let srv = given("req/url-query/url-single");
    get(&srv.path_query("/api/url", "age", "young")).await.expect_status_ok();
}

#[async_std::test]
async fn should_fail_when_missing_query() {
    let srv = given("req/url-query/url-single");
    get(&srv.path("/api/url")).await.expect_status_not_found();
}

#[async_std::test]
async fn should_fail_when_incorrect_path() {
    let srv = given("req/url-query/url-single");
    get(&srv.path_query("/api/not-url", "age", "young")).await.expect_status_not_found();
}

#[async_std::test]
async fn should_fail_when_incorrect_query() {
    let srv = given("req/url-query/url-single");
    get(&srv.path_query("/api/url", "age", "old")).await.expect_status_not_found();
}

#[async_std::test]
async fn should_map_request_url_and_many_query() {
    let srv = given("req/url-query/url-many");
    get(&srv.path_queries("/api/url", ("age", "young"), ("city", "paris"))).await.expect_status_ok();
}

#[async_std::test]
async fn should_fail_when_incorrect_uri_with_many_queries() {
    let srv = given("req/url-query/url-many");
    get(&srv.path_queries("/api/not-url", ("age", "young"), ("city", "paris"))).await.expect_status_not_found();
}

#[async_std::test]
async fn should_fail_when_one_of_queries_does_not_match() {
    let srv = given("req/url-query/url-many");
    get(&srv.path_queries("/api/url", ("age", "old"), ("city", "paris"))).await.expect_status_not_found();
    get(&srv.path_queries("/api/url", ("age", "young"), ("city", "lyon"))).await.expect_status_not_found();
    get(&srv.path_query("/api/url", "age", "young")).await.expect_status_not_found();
    get(&srv.path_query("/api/url", "city", "paris")).await.expect_status_not_found();
}
