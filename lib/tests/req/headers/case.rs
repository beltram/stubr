use asserhttp::*;
use surf::get;

#[async_std::test]
#[stubr::mock("req/headers/equal/string.json")]
async fn should_default_to_case_sensitive() {
    get(stubr.uri())
        .header("Content-Type", "Application/Json")
        .await
        .expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/headers/case/insensitive.json")]
async fn should_support_case_insensitive() {
    get(stubr.uri())
        .header("Content-Type", "Application/Json")
        .await
        .expect_status_ok();
    get(stubr.uri())
        .header("Content-Type", "application/json")
        .await
        .expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/headers/case/insensitive.json")]
async fn insensitive_should_fail_when_invalid_value() {
    get(stubr.uri())
        .header("Content-Type", "application/xml")
        .await
        .expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/headers/case/insensitive.json")]
async fn insensitive_should_fail_when_invalid_key() {
    get(stubr.uri())
        .header("Not-Content-Type", "application/json")
        .await
        .expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/headers/case/insensitive.json")]
async fn insensitive_should_fail_when_missing() {
    get(stubr.uri()).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/headers/case/insensitive-many.json")]
async fn should_support_many_case_insensitive() {
    get(stubr.uri())
        .header("Content-Type", "Application/Json")
        .header("Accept", "Application/Json")
        .await
        .expect_status_ok();
    get(stubr.uri())
        .header("Content-Type", "application/json")
        .header("Accept", "Application/Json")
        .await
        .expect_status_ok();
    get(stubr.uri())
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .await
        .expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/headers/case/insensitive-many.json")]
async fn should_fail_with_many_case_insensitive_string_value_when_one_of_does_not_match() {
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
#[stubr::mock("req/headers/case/sensitive.json")]
async fn should_support_explicit_case_sensitive() {
    get(stubr.uri())
        .header("Content-Type", "Application/Json")
        .await
        .expect_status_not_found();
    get(stubr.uri())
        .header("Content-Type", "application/json")
        .await
        .expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/headers/case/sensitive.json")]
async fn sensitive_should_fail_when_invalid_value() {
    get(stubr.uri())
        .header("Content-Type", "application/xml")
        .await
        .expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/headers/case/sensitive.json")]
async fn sensitive_should_fail_when_invalid_key() {
    get(stubr.uri())
        .header("Not-Content-Type", "application/json")
        .await
        .expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/headers/case/sensitive.json")]
async fn sensitive_should_fail_when_missing() {
    get(stubr.uri()).await.expect_status_not_found();
}
