use surf::get;

use crate::utils::*;

#[async_std::test]
async fn should_default_to_case_sensitive() {
    let srv = given("req/query/equal/string");
    get(&srv.query("age", "YOUNG")).await.unwrap()
        .assert_not_found();
}

#[async_std::test]
async fn should_support_case_insensitive() {
    let srv = given("req/query/case/insensitive");
    get(&srv.query("age", "YOUNG")).await.unwrap().assert_ok();
    get(&srv.query("age", "young")).await.unwrap().assert_ok();
}

#[async_std::test]
async fn insensitive_should_fail_when_invalid_key() {
    let srv = given("req/query/case/insensitive");
    get(&srv.query("not-age", "young")).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn insensitive_should_fail_when_missing() {
    let srv = given("req/query/case/insensitive");
    get(&srv.url()).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_support_many_case_insensitive() {
    let srv = given("req/query/case/insensitive-many");
    get(&srv.queries(("age", "YOUNG"), ("city", "PARIS"))).await.unwrap().assert_ok();
    get(&srv.queries(("age", "young"), ("city", "PARIS"))).await.unwrap().assert_ok();
    get(&srv.queries(("age", "young"), ("city", "paris"))).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_fail_with_many_case_insensitive_string_value_when_one_of_does_not_match() {
    let srv = given("req/query/case/insensitive-many");
    get(&srv.queries(("age", "old"), ("city", "paris"))).await.unwrap().assert_not_found();
    get(&srv.queries(("age", "young"), ("city", "lyon"))).await.unwrap().assert_not_found();
    get(&srv.query("age", "young")).await.unwrap().assert_not_found();
    get(&srv.query("city", "paris")).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_support_explicit_case_sensitive() {
    let srv = given("req/query/case/sensitive");
    get(&srv.query("age", "YOUNG")).await.unwrap().assert_not_found();
    get(&srv.query("age", "young")).await.unwrap().assert_ok();
}

#[async_std::test]
async fn sensitive_should_fail_when_invalid_key() {
    let srv = given("req/query/case/sensitive");
    get(&srv.query("not-age", "young")).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn sensitive_should_fail_when_missing() {
    let srv = given("req/query/case/sensitive");
    get(&srv.url()).await.unwrap().assert_not_found();
}