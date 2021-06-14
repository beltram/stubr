use asserhttp::*;
use surf::get;

use crate::utils::*;

#[async_std::test]
async fn should_map_request_when_header_matches() {
    let srv = given("req/headers/matches/single");
    get(&srv.uri()).header("Content-Type", "application/json").await.expect_status_ok();
    get(&srv.uri()).header("Content-Type", "application/xml").await.expect_status_ok();
}

#[async_std::test]
async fn should_fail_when_absent() {
    let srv = given("req/headers/matches/single");
    get(&srv.uri()).await.expect_status_not_found();
}

#[async_std::test]
async fn should_fail_when_regex_not_respected() {
    let srv = given("req/headers/matches/single");
    get(&srv.uri()).header("Content-Type", "text/plain").await.expect_status_not_found();
    get(&srv.uri()).header("Content-Type", "app/json").await.expect_status_not_found();
}

#[async_std::test]
async fn should_map_request_when_many_header_matches() {
    let srv = given("req/headers/matches/many");
    get(&srv.uri())
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .await
        .expect_status_ok();
}

#[async_std::test]
async fn should_fail_when_one_of_header_regex_not_respected() {
    let srv = given("req/headers/matches/many");
    get(&srv.uri())
        .header("Content-Type", "app/json")
        .header("Accept", "application/json")
        .await.expect_status_not_found();
    get(&srv.uri())
        .header("Content-Type", "application/json")
        .header("Accept", "app/json")
        .await.expect_status_not_found();
    get(&srv.uri())
        .header("Content-Type", "app/json")
        .header("Accept", "app/json")
        .await.expect_status_not_found();
    get(&srv.uri()).header("Content-Type", "application/json").await.expect_status_not_found();
    get(&srv.uri()).header("Accept", "application/json").await.expect_status_not_found();
    get(&srv.uri()).await.expect_status_not_found();
}

#[async_std::test]
async fn negative_should_map_request_when_header_matches() {
    let srv = given("req/headers/matches/negative");
    get(&srv.uri()).header("Content-Type", "application/json").await.expect_status_ok();
    get(&srv.uri()).header("Content-Type", "any").await.expect_status_ok();
}

#[async_std::test]
async fn negative_should_fail_when_absent() {
    let srv = given("req/headers/matches/negative");
    get(&srv.uri()).await.expect_status_not_found();
}

#[async_std::test]
async fn negative_should_fail_when_regex_not_respected() {
    let srv = given("req/headers/matches/negative");
    get(&srv.uri()).header("Content-Type", "text/plain").await.expect_status_not_found();
}

#[async_std::test]
async fn negative_should_map_request_when_many_header_matches() {
    let srv = given("req/headers/matches/negative-many");
    get(&srv.uri())
        .header("Content-Type", "application/json")
        .header("X-Some", "application/json")
        .await
        .expect_status_ok();
}

// using 'X-Some' because surf adds by default a relaxed Accept header
#[async_std::test]
async fn negative_should_fail_when_one_of_header_regex_not_respected() {
    let srv = given("req/headers/matches/negative-many");
    get(&srv.uri())
        .header("Content-Type", "application/json")
        .header("X-Some", "text/plain")
        .await
        .expect_status_not_found();
    get(&srv.uri())
        .header("Content-Type", "text/plain")
        .header("X-Some", "application/json")
        .await
        .expect_status_not_found();
    get(&srv.uri())
        .header("Content-Type", "text/plain")
        .header("X-Some", "text/plain")
        .await
        .expect_status_not_found();
    get(&srv.uri()).header("Content-Type", "application/json").await.expect_status_not_found();
    get(&srv.uri()).header("X-Some", "application/json").await.expect_status_not_found();
    get(&srv.uri()).await.expect_status_not_found();
}
