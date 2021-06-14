use asserhttp::*;
use surf::get;

use crate::utils::*;

/// tested against query parameters but would work the same way for header parameters
/// since exact same code is used
#[async_std::test]
async fn equal_should_have_precedence_over_contains() {
    let srv = given("req/query/precedence/eq-contains");
    get(&srv.query("age", "young")).await.expect_status_ok();
    get(&srv.query("age", "old")).await.expect_status_not_found();
    get(&srv.query("age", "urluberlu")).await.expect_status_not_found();
}

#[async_std::test]
async fn equal_should_have_precedence_over_matches() {
    let srv = given("req/query/precedence/eq-matches");
    get(&srv.query("age", "20")).await.expect_status_ok();
    get(&srv.query("age", "21")).await.expect_status_not_found();
    get(&srv.query("age", "twenty")).await.expect_status_not_found();
}

#[async_std::test]
async fn equal_should_have_precedence_over_negative_matches() {
    let srv = given("req/query/precedence/eq-neg-matches");
    get(&srv.query("age", "young")).await.expect_status_ok();
    get(&srv.query("age", "old")).await.expect_status_not_found();
    get(&srv.query("age", "20")).await.expect_status_not_found();
}

#[async_std::test]
async fn equal_case_sensitive_should_have_precedence_over_contains() {
    let srv = given("req/query/precedence/eq-case-sensitive-contains");
    get(&srv.query("age", "young")).await.expect_status_ok();
    get(&srv.query("age", "YOUNG")).await.expect_status_not_found();
    get(&srv.query("age", "urluberlu")).await.expect_status_not_found();
}

#[async_std::test]
async fn equal_case_sensitive_should_have_precedence_over_matches() {
    let srv = given("req/query/precedence/eq-case-sensitive-matches");
    get(&srv.query("age", "old")).await.expect_status_ok();
    get(&srv.query("age", "OLD")).await.expect_status_not_found();
    get(&srv.query("age", "young")).await.expect_status_not_found();
}

#[async_std::test]
async fn equal_case_sensitive_should_have_precedence_over_negative_matches() {
    let srv = given("req/query/precedence/eq-case-sensitive-neg-matches");
    get(&srv.query("age", "young")).await.expect_status_ok();
    get(&srv.query("age", "YOUNG")).await.expect_status_not_found();
    get(&srv.query("age", "old")).await.expect_status_not_found();
}

#[async_std::test]
async fn equal_case_insensitive_should_have_precedence_over_contains() {
    let srv = given("req/query/precedence/eq-case-insensitive-contains");
    get(&srv.query("age", "young")).await.expect_status_ok();
    get(&srv.query("age", "YOUNG")).await.expect_status_ok();
    get(&srv.query("age", "urluberlu")).await.expect_status_not_found();
}

#[async_std::test]
async fn equal_case_insensitive_should_have_precedence_over_matches() {
    let srv = given("req/query/precedence/eq-case-insensitive-matches");
    get(&srv.query("age", "old")).await.expect_status_ok();
    get(&srv.query("age", "OLD")).await.expect_status_ok();
    get(&srv.query("age", "young")).await.expect_status_not_found();
}

#[async_std::test]
async fn equal_case_insensitive_should_have_precedence_over_negative_matches() {
    let srv = given("req/query/precedence/eq-case-insensitive-neg-matches");
    get(&srv.query("age", "young")).await.expect_status_ok();
    get(&srv.query("age", "YOUNG")).await.expect_status_ok();
    get(&srv.query("age", "old")).await.expect_status_not_found();
}

#[async_std::test]
async fn contains_should_have_precedence_over_matches() {
    let srv = given("req/query/precedence/contains-matches");
    get(&srv.query("age", "young")).await.expect_status_ok();
    get(&srv.query("age", "aaa")).await.expect_status_not_found();
    get(&srv.query("age", "www")).await.expect_status_not_found();
}

#[async_std::test]
async fn contains_should_have_precedence_over_negative_matches() {
    let srv = given("req/query/precedence/contains-neg-matches");
    get(&srv.query("age", "young")).await.expect_status_ok();
    get(&srv.query("age", "aaa")).await.expect_status_not_found();
    get(&srv.query("age", "zzz")).await.expect_status_not_found();
}