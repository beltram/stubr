use asserhttp::*;
use surf::get;

#[async_std::test]
#[stubr::mock("req/url/url-path.json")]
async fn should_map_request_url_path_uri() {
    get(stubr.path("/api/exact-uri")).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/url/url-path.json")]
async fn should_not_match_when_url_path_not_exact() {
    get(stubr.path("/api/not-exact-uri")).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/url/no-url.json")]
async fn should_not_fail_when_no_url() {
    get(stubr.uri()).await.expect_status_ok();
}
