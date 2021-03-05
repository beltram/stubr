use surf::get;

use crate::utils::*;

#[async_std::test]
async fn should_match_when_header_absent() {
    let srv = given("req/query/absence/absent");
    get(&srv.url()).await.unwrap().assert_ok();
    get(&srv.query("age", "42")).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_match_when_header_present() {
    let srv = given("req/query/absence/present");
    get(&srv.url()).await.unwrap().assert_not_found();
    get(&srv.query("age", "42")).await.unwrap().assert_ok();
}