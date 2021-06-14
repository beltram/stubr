use asserhttp::*;
use surf::get;

use crate::utils::*;

#[async_std::test]
async fn should_map_request_url_path_pattern_uri() {
    let srv = given("req/url/url-path-pattern");
    get(&srv.path("/api/regex-uri/abcd")).await.expect_status_ok();
}

#[async_std::test]
async fn should_not_match_when_url_path_pattern_not_satisfied() {
    let srv = given("req/url/url-path-pattern");
    get(&srv.path("/api/regex-uri/1234")).await.expect_status_not_found();
}