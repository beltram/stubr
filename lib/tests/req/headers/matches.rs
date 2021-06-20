use asserhttp::*;
use surf::get;

#[async_std::test]
#[stubr::mock("req/headers/matches/single.json")]
async fn should_map_request_when_header_matches() {
    get(stubr.uri()).header("Content-Type", "application/json").await.expect_status_ok();
    get(stubr.uri()).header("Content-Type", "application/xml").await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/headers/matches/single.json")]
async fn should_fail_when_absent() {
    get(stubr.uri()).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/headers/matches/single.json")]
async fn should_fail_when_regex_not_respected() {
    get(stubr.uri()).header("Content-Type", "text/plain").await.expect_status_not_found();
    get(stubr.uri()).header("Content-Type", "app/json").await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/headers/matches/many.json")]
async fn should_map_request_when_many_header_matches() {
    get(stubr.uri())
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .await
        .expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/headers/matches/many.json")]
async fn should_fail_when_one_of_header_regex_not_respected() {
    get(stubr.uri())
        .header("Content-Type", "app/json")
        .header("Accept", "application/json")
        .await.expect_status_not_found();
    get(stubr.uri())
        .header("Content-Type", "application/json")
        .header("Accept", "app/json")
        .await.expect_status_not_found();
    get(stubr.uri())
        .header("Content-Type", "app/json")
        .header("Accept", "app/json")
        .await.expect_status_not_found();
    get(stubr.uri()).header("Content-Type", "application/json").await.expect_status_not_found();
    get(stubr.uri()).header("Accept", "application/json").await.expect_status_not_found();
    get(stubr.uri()).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/headers/matches/negative.json")]
async fn negative_should_map_request_when_header_matches() {
    get(stubr.uri()).header("Content-Type", "application/json").await.expect_status_ok();
    get(stubr.uri()).header("Content-Type", "any").await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/headers/matches/negative.json")]
async fn negative_should_fail_when_absent() {
    get(stubr.uri()).await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/headers/matches/negative.json")]
async fn negative_should_fail_when_regex_not_respected() {
    get(stubr.uri()).header("Content-Type", "text/plain").await.expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/headers/matches/negative-many.json")]
async fn negative_should_map_request_when_many_header_matches() {
    get(stubr.uri())
        .header("Content-Type", "application/json")
        .header("X-Some", "application/json")
        .await
        .expect_status_ok();
}

// using 'X-Some' because surf adds by default a relaxed Accept header
#[async_std::test]
#[stubr::mock("req/headers/matches/negative-many.json")]
async fn negative_should_fail_when_one_of_header_regex_not_respected() {
    get(stubr.uri())
        .header("Content-Type", "application/json")
        .header("X-Some", "text/plain")
        .await
        .expect_status_not_found();
    get(stubr.uri())
        .header("Content-Type", "text/plain")
        .header("X-Some", "application/json")
        .await
        .expect_status_not_found();
    get(stubr.uri())
        .header("Content-Type", "text/plain")
        .header("X-Some", "text/plain")
        .await
        .expect_status_not_found();
    get(stubr.uri()).header("Content-Type", "application/json").await.expect_status_not_found();
    get(stubr.uri()).header("X-Some", "application/json").await.expect_status_not_found();
    get(stubr.uri()).await.expect_status_not_found();
}
