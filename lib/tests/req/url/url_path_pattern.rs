use asserhttp::*;
use surf::get;

use crate::utils::*;

#[async_std::test]
#[stubr::mock("req/url/url-path-pattern.json")]
async fn should_map_request_url_path_pattern_uri() {
    get(stubr.path("/api/regex-uri/abcd")).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/url/url-path-pattern.json")]
async fn should_not_match_when_url_path_pattern_not_satisfied() {
    get(stubr.path("/api/regex-uri/1234")).await.expect_status_not_found();
}