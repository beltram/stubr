use asserhttp::*;
use surf::get;

use crate::utils::*;

/// tested against query parameters but would work the same way for header parameters
/// since exact same code is used
#[async_std::test]
#[stubr::mock("req/query/precedence/eq-contains.json")]
async fn equal_should_have_precedence_over_contains() {
    get(stubr.query("age", "young")).await.expect_status_ok();
    get(stubr.query("age", "old")).await.expect_status_not_found();
    get(stubr.query("age", "urluberlu")).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/query/precedence/eq-matches.json")]
async fn equal_should_have_precedence_over_matches() {
    get(stubr.query("age", "20")).await.expect_status_ok();
    get(stubr.query("age", "21")).await.expect_status_not_found();
    get(stubr.query("age", "twenty")).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/query/precedence/eq-neg-matches.json")]
async fn equal_should_have_precedence_over_negative_matches() {
    get(stubr.query("age", "young")).await.expect_status_ok();
    get(stubr.query("age", "old")).await.expect_status_not_found();
    get(stubr.query("age", "20")).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/query/precedence/eq-case-sensitive-contains.json")]
async fn equal_case_sensitive_should_have_precedence_over_contains() {
    get(stubr.query("age", "young")).await.expect_status_ok();
    get(stubr.query("age", "YOUNG")).await.expect_status_not_found();
    get(stubr.query("age", "urluberlu")).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/query/precedence/eq-case-sensitive-matches.json")]
async fn equal_case_sensitive_should_have_precedence_over_matches() {
    get(stubr.query("age", "old")).await.expect_status_ok();
    get(stubr.query("age", "OLD")).await.expect_status_not_found();
    get(stubr.query("age", "young")).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/query/precedence/eq-case-sensitive-neg-matches.json")]
async fn equal_case_sensitive_should_have_precedence_over_negative_matches() {
    get(stubr.query("age", "young")).await.expect_status_ok();
    get(stubr.query("age", "YOUNG")).await.expect_status_not_found();
    get(stubr.query("age", "old")).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/query/precedence/eq-case-insensitive-contains.json")]
async fn equal_case_insensitive_should_have_precedence_over_contains() {
    get(stubr.query("age", "young")).await.expect_status_ok();
    get(stubr.query("age", "YOUNG")).await.expect_status_ok();
    get(stubr.query("age", "urluberlu")).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/query/precedence/eq-case-insensitive-matches.json")]
async fn equal_case_insensitive_should_have_precedence_over_matches() {
    get(stubr.query("age", "old")).await.expect_status_ok();
    get(stubr.query("age", "OLD")).await.expect_status_ok();
    get(stubr.query("age", "young")).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/query/precedence/eq-case-insensitive-neg-matches.json")]
async fn equal_case_insensitive_should_have_precedence_over_negative_matches() {
    get(stubr.query("age", "young")).await.expect_status_ok();
    get(stubr.query("age", "YOUNG")).await.expect_status_ok();
    get(stubr.query("age", "old")).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/query/precedence/contains-matches.json")]
async fn contains_should_have_precedence_over_matches() {
    get(stubr.query("age", "young")).await.expect_status_ok();
    get(stubr.query("age", "aaa")).await.expect_status_not_found();
    get(stubr.query("age", "www")).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/query/precedence/contains-neg-matches.json")]
async fn contains_should_have_precedence_over_negative_matches() {
    get(stubr.query("age", "young")).await.expect_status_ok();
    get(stubr.query("age", "aaa")).await.expect_status_not_found();
    get(stubr.query("age", "zzz")).await.expect_status_not_found();
}
