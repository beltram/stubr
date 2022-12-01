use asserhttp::*;
use surf::get;

use crate::utils::*;

#[async_std::test]
#[stubr::mock("req/query/equal/string.json")]
async fn should_default_to_case_sensitive() {
    get(stubr.query("age", "YOUNG")).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/query/case/insensitive.json")]
async fn should_support_case_insensitive() {
    get(stubr.query("age", "YOUNG")).await.expect_status_ok();
    get(stubr.query("age", "young")).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/query/case/insensitive.json")]
async fn insensitive_should_fail_when_invalid_key() {
    get(stubr.query("not-age", "young")).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/query/case/insensitive.json")]
async fn insensitive_should_fail_when_missing() {
    get(stubr.uri()).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/query/case/insensitive-many.json")]
async fn should_support_many_case_insensitive() {
    get(stubr.queries(("age", "YOUNG"), ("city", "PARIS"))).await.expect_status_ok();
    get(stubr.queries(("age", "young"), ("city", "PARIS"))).await.expect_status_ok();
    get(stubr.queries(("age", "young"), ("city", "paris"))).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/query/case/insensitive-many.json")]
async fn should_fail_with_many_case_insensitive_string_value_when_one_of_does_not_match() {
    get(stubr.queries(("age", "old"), ("city", "paris")))
        .await
        .expect_status_not_found();
    get(stubr.queries(("age", "young"), ("city", "lyon")))
        .await
        .expect_status_not_found();
    get(stubr.query("age", "young")).await.expect_status_not_found();
    get(stubr.query("city", "paris")).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/query/case/sensitive.json")]
async fn should_support_explicit_case_sensitive() {
    get(stubr.query("age", "YOUNG")).await.expect_status_not_found();
    get(stubr.query("age", "young")).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/query/case/sensitive.json")]
async fn sensitive_should_fail_when_invalid_key() {
    get(stubr.query("not-age", "young")).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/query/case/sensitive.json")]
async fn sensitive_should_fail_when_missing() {
    get(stubr.uri()).await.expect_status_not_found();
}
