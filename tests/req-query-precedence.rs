use surf::get;

use crate::utils::*;

mod utils;

#[async_std::test]
async fn equal_should_have_precedence_over_contains() {
    let srv = given("req/query/precedence/eq-contains");
    get(&srv.query("age", "young")).await.unwrap().assert_ok();
    get(&srv.query("age", "old")).await.unwrap().assert_not_found();
    get(&srv.query("age", "urluberlu")).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn equal_should_have_precedence_over_matches() {
    let srv = given("req/query/precedence/eq-matches");
    get(&srv.query("age", "20")).await.unwrap().assert_ok();
    get(&srv.query("age", "21")).await.unwrap().assert_not_found();
    get(&srv.query("age", "twenty")).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn equal_should_have_precedence_over_negative_matches() {
    let srv = given("req/query/precedence/eq-neg-matches");
    get(&srv.query("age", "young")).await.unwrap().assert_ok();
    get(&srv.query("age", "old")).await.unwrap().assert_not_found();
    get(&srv.query("age", "20")).await.unwrap().assert_not_found();
}