use asserhttp::*;
use surf::get;

#[async_std::test]
#[stubr::mock("req/headers/absence/absent.json")]
async fn should_match_when_header_absent() {
    get(stubr.uri()).await.expect_status_ok();
    get(stubr.uri()).header("X-Maybe", "any").await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/headers/absence/present.json")]
async fn should_match_when_header_present() {
    get(stubr.uri()).await.expect_status_not_found();
    get(stubr.uri()).header("X-Maybe", "any").await.expect_status_ok();
}