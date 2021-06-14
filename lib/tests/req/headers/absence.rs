use asserhttp::*;
use surf::get;

use crate::utils::*;

#[async_std::test]
async fn should_match_when_header_absent() {
    let srv = given("req/headers/absence/absent");
    get(&srv.uri()).await.expect_status_ok();
    get(&srv.uri()).header("X-Maybe", "any").await.expect_status_not_found();
}

#[async_std::test]
async fn should_match_when_header_present() {
    let srv = given("req/headers/absence/present");
    get(&srv.uri()).await.expect_status_not_found();
    get(&srv.uri()).header("X-Maybe", "any").await.expect_status_ok();
}