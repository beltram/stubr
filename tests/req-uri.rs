use async_std::task::block_on;
use surf::get;

use crate::utils::*;

mod utils;

#[async_std::test]
async fn should_map_request_url_path_uri() {
    let srv = given("req/url/url-path");
    get(&srv.path("/api/exact-uri")).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_not_match_when_url_path_not_exact() {
    let srv = given("req/url/url-path");
    get(&srv.path("/api/not-exact-uri")).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_map_request_url_path_pattern_uri() {
    let srv = given("req/url/url-path-pattern");
    get(&srv.path("/api/regex-uri/abcd")).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_not_match_when_url_path_pattern_not_satisfied() {
    let srv = given("req/url/url-path-pattern");
    get(&srv.path("/api/regex-uri/1234")).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_not_fail_when_no_url() {
    let srv = given("req/url/no-url");
    get(&srv.uri()).await.unwrap().assert_ok();
}
