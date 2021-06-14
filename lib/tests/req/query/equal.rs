use asserhttp::*;
use surf::get;

use crate::utils::*;

#[async_std::test]
async fn should_map_request_exact_string_query() {
    let srv = given("req/query/equal/string");
    get(&srv.query("age", "young")).await.expect_status_ok();
    get(&srv.query("age", "young")).await.expect_status_ok();
}

#[async_std::test]
async fn should_fail_when_incorrect_string_value() {
    let srv = given("req/query/equal/string");
    get(&srv.query("age", "old")).await.expect_status_not_found();
}

#[async_std::test]
async fn should_fail_when_invalid_key() {
    let srv = given("req/query/equal/string");
    get(&srv.query("not-age", "young")).await.expect_status_not_found();
}

#[async_std::test]
async fn should_fail_when_missing() {
    let srv = given("req/query/equal/string");
    get(&srv.uri()).await.expect_status_not_found();
}

#[async_std::test]
async fn should_map_request_many_exact_string_query() {
    let srv = given("req/query/equal/string-many");
    get(&srv.queries(("age", "young"), ("city", "paris"))).await.expect_status_ok();
}

#[async_std::test]
async fn should_fail_with_many_exact_string_value_when_one_of_does_not_match() {
    let srv = given("req/query/equal/string-many");
    get(&srv.queries(("age", "old"), ("city", "paris"))).await.expect_status_not_found();
    get(&srv.queries(("age", "young"), ("city", "lyon"))).await.expect_status_not_found();
    get(&srv.query("age", "young")).await.expect_status_not_found();
    get(&srv.query("city", "paris")).await.expect_status_not_found();
}

#[async_std::test]
async fn should_map_request_exact_int_value() {
    let srv = given("req/query/equal/int");
    get(&srv.query("age", "42")).await.expect_status_ok();
}

#[async_std::test]
async fn should_fail_when_incorrect_int_value() {
    let srv = given("req/query/equal/int");
    get(&srv.query("age", "43")).await.expect_status_not_found();
}

#[async_std::test]
async fn should_fail_when_not_an_int_value() {
    let srv = given("req/query/equal/int");
    get(&srv.query("age", "string")).await.expect_status_not_found();
}

#[async_std::test]
async fn should_map_request_exact_bool_value() {
    let srv = given("req/query/equal/bool");
    get(&srv.query("age", "true")).await.expect_status_ok();
}

#[async_std::test]
async fn should_fail_when_incorrect_bool_value() {
    let srv = given("req/query/equal/bool");
    get(&srv.query("age", "false")).await.expect_status_not_found();
}

#[async_std::test]
async fn should_fail_when_not_a_bool() {
    let srv = given("req/query/equal/bool");
    get(&srv.query("age", "42")).await.expect_status_not_found();
}