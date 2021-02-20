use surf::get;

use crate::utils::*;

#[async_std::test]
async fn should_map_request_url_and_query() {
    let srv = given("req/url-query/url-single");
    get(&srv.path_query("/api/url", "age", "young")).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_fail_when_missing_query() {
    let srv = given("req/url-query/url-single");
    get(&srv.path("/api/url")).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_fail_when_incorrect_path() {
    let srv = given("req/url-query/url-single");
    get(&srv.path_query("/api/not-url", "age", "young")).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_fail_when_incorrect_query() {
    let srv = given("req/url-query/url-single");
    get(&srv.path_query("/api/url", "age", "old")).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_map_request_url_and_many_query() {
    let srv = given("req/url-query/url-many");
    get(&srv.path_queries("/api/url", ("age", "young"), ("city", "paris"))).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_fail_when_incorrect_uri_with_many_queries() {
    let srv = given("req/url-query/url-many");
    get(&srv.path_queries("/api/not-url", ("age", "young"), ("city", "paris"))).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_fail_when_one_of_queries_does_not_match() {
    let srv = given("req/url-query/url-many");
    get(&srv.path_queries("/api/url", ("age", "old"), ("city", "paris"))).await.unwrap().assert_not_found();
    get(&srv.path_queries("/api/url", ("age", "young"), ("city", "lyon"))).await.unwrap().assert_not_found();
    get(&srv.path_query("/api/url", "age", "young")).await.unwrap().assert_not_found();
    get(&srv.path_query("/api/url", "city", "paris")).await.unwrap().assert_not_found();
}
