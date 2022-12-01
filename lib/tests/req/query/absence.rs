use asserhttp::*;
use surf::get;

use crate::utils::*;

#[async_std::test]
#[stubr::mock("req/query/absence/absent.json")]
async fn should_match_when_header_absent() {
    get(stubr.uri()).await.expect_status_ok();
    get(stubr.query("age", "42")).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/query/absence/present.json")]
async fn should_match_when_header_present() {
    get(stubr.uri()).await.expect_status_not_found();
    get(stubr.query("age", "42")).await.expect_status_ok();
}
