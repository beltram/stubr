use surf::get;

use crate::utils::*;

mod utils;

#[async_std::test]
async fn should_match_when_header_absent() {
    let srv = given("req/headers/absence/absent");
    get(&srv.uri()).await.unwrap().assert_ok();
    get(&srv.uri()).header("X-Maybe", "any").await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_match_when_header_present() {
    let srv = given("req/headers/absence/present");
    get(&srv.uri()).await.unwrap().assert_not_found();
    get(&srv.uri()).header("X-Maybe", "any").await.unwrap().assert_ok();
}