use asserhttp::*;
use surf::get;

use crate::utils::*;

#[async_std::test]
#[stubr::mock("req/query/equal/string.json")]
async fn should_map_request_exact_string_query() {
    get(stubr.query("age", "young")).await.expect_status_ok();
    get(stubr.query("age", "young")).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/query/equal/string.json")]
async fn should_fail_when_incorrect_string_value() {
    get(stubr.query("age", "old")).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/query/equal/string.json")]
async fn should_fail_when_invalid_key() {
    get(stubr.query("not-age", "young")).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/query/equal/string.json")]
async fn should_fail_when_missing() {
    get(stubr.uri()).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/query/equal/string-many.json")]
async fn should_map_request_many_exact_string_query() {
    get(stubr.queries(("age", "young"), ("city", "paris"))).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/query/equal/string-many.json")]
async fn should_fail_with_many_exact_string_value_when_one_of_does_not_match() {
    get(stubr.queries(("age", "old"), ("city", "paris"))).await.expect_status_not_found();
    get(stubr.queries(("age", "young"), ("city", "lyon"))).await.expect_status_not_found();
    get(stubr.query("age", "young")).await.expect_status_not_found();
    get(stubr.query("city", "paris")).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/query/equal/int.json")]
async fn should_map_request_exact_int_value() {
    get(stubr.query("age", "42")).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/query/equal/int.json")]
async fn should_fail_when_incorrect_int_value() {
    get(stubr.query("age", "43")).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/query/equal/int.json")]
async fn should_fail_when_not_an_int_value() {
    get(stubr.query("age", "string")).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/query/equal/bool.json")]
async fn should_map_request_exact_bool_value() {
    get(stubr.query("age", "true")).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/query/equal/bool.json")]
async fn should_fail_when_incorrect_bool_value() {
    get(stubr.query("age", "false")).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/query/equal/bool.json")]
async fn should_fail_when_not_a_bool() {
    get(stubr.query("age", "42")).await.expect_status_not_found();
}