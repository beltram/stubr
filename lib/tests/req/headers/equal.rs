use asserhttp::*;
use surf::get;

#[async_std::test]
#[stubr::mock("req/headers/equal/string.json")]
async fn should_map_request_exact_string_value() {
    get(stubr.uri())
        .header("Content-Type", "application/json")
        .await
        .expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/headers/equal/string.json")]
async fn should_fail_when_incorrect_string_value() {
    get(stubr.uri())
        .header("Content-Type", "application/xml")
        .await
        .expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/headers/equal/string.json")]
async fn should_fail_when_invalid_key() {
    get(stubr.uri())
        .header("Not-Content-Type", "application/json")
        .await
        .expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/headers/equal/string.json")]
async fn should_fail_when_missing() {
    get(stubr.uri()).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/headers/equal/string-many.json")]
async fn should_map_request_many_exact_string_value() {
    get(stubr.uri())
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .await
        .expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/headers/equal/string-many.json")]
async fn should_fail_with_many_exact_string_value_when_one_of_does_not_match() {
    get(stubr.uri())
        .header("Content-Type", "application/xml")
        .header("Accept", "application/json")
        .await
        .expect_status_not_found();
    get(stubr.uri())
        .header("Content-Type", "application/json")
        .header("Accept", "application/xml")
        .await
        .expect_status_not_found();
    get(stubr.uri())
        .header("Content-Type", "application/json")
        .await
        .expect_status_not_found();
    get(stubr.uri())
        .header("Accept", "application/json")
        .await
        .expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/headers/equal/int.json")]
async fn should_map_request_exact_int_value() {
    get(stubr.uri()).header("Content-Type", "42").await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/headers/equal/int.json")]
async fn should_fail_when_incorrect_int_value() {
    get(stubr.uri()).header("Content-Type", "43").await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/headers/equal/int.json")]
async fn should_fail_when_not_an_int_value() {
    get(stubr.uri())
        .header("Content-Type", "application/json")
        .await
        .expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/headers/equal/bool.json")]
async fn should_map_request_exact_bool_value() {
    get(stubr.uri()).header("Content-Type", "true").await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/headers/equal/bool.json")]
async fn should_fail_when_incorrect_bool_value() {
    get(stubr.uri()).header("Content-Type", "false").await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/headers/equal/bool.json")]
async fn should_fail_when_not_an_bool_value() {
    get(stubr.uri())
        .header("Content-Type", "application/json")
        .await
        .expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/headers/equal/string-int.json")]
async fn should_map_request_many_exact_string_and_int_value() {
    get(stubr.uri())
        .header("Content-Type", "application/json")
        .header("Accept", "true")
        .await
        .expect_status_ok();
}
