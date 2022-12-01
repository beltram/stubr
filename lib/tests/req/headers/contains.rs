use asserhttp::*;
use surf::get;

#[async_std::test]
#[stubr::mock("req/headers/equal/string.json")]
async fn should_not_default_to_contains() {
    get(stubr.uri()).header("Content-Type", "json").await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/headers/contains/single.json")]
async fn should_support_contains() {
    get(stubr.uri())
        .header("Content-Type", "application/json")
        .await
        .expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/headers/contains/single.json")]
async fn should_fail_when_does_not_contain() {
    get(stubr.uri())
        .header("Content-Type", "application/xml")
        .await
        .expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/headers/contains/single.json")]
async fn should_fail_when_invalid_key() {
    get(stubr.uri())
        .header("Not-Content-Type", "application/json")
        .await
        .expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/headers/contains/single.json")]
async fn should_fail_when_missing() {
    get(stubr.uri()).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/headers/contains/many.json")]
async fn should_support_many_contains() {
    get(stubr.uri())
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .await
        .expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/headers/contains/many.json")]
async fn should_fail_when_one_of_does_not_contains() {
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
#[stubr::mock("req/headers/contains/begin.json")]
async fn should_support_contains_begin() {
    get(stubr.uri())
        .header("Content-Type", "application/json")
        .await
        .expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/headers/contains/middle.json")]
async fn should_support_contains_middle() {
    get(stubr.uri())
        .header("Content-Type", "application/json")
        .await
        .expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/headers/contains/end.json")]
async fn should_support_contains_end() {
    get(stubr.uri())
        .header("Content-Type", "application/json")
        .await
        .expect_status_ok();
}
