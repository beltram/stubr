use surf::get;

use crate::utils::*;

#[async_std::test]
async fn should_match_when_header_absent() {
    let srv = given("req/headers/absence/absent");
    get(&srv.url()).await.unwrap().assert_ok();
    get(&srv.url()).header("X-Maybe", "any").await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_match_when_header_present() {
    let srv = given("req/headers/absence/present");
    get(&srv.url()).await.unwrap().assert_not_found();
    get(&srv.url()).header("X-Maybe", "any").await.unwrap().assert_ok();
}