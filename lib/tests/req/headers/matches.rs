use surf::get;

use crate::utils::*;

#[async_std::test]
async fn should_map_request_when_header_matches() {
    let srv = given("req/headers/matches/single");
    get(&srv.uri()).header("Content-Type", "application/json").await.unwrap().assert_ok();
    get(&srv.uri()).header("Content-Type", "application/xml").await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_fail_when_absent() {
    let srv = given("req/headers/matches/single");
    get(&srv.uri()).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_fail_when_regex_not_respected() {
    let srv = given("req/headers/matches/single");
    get(&srv.uri()).header("Content-Type", "text/plain").await.unwrap().assert_not_found();
    get(&srv.uri()).header("Content-Type", "app/json").await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_map_request_when_many_header_matches() {
    let srv = given("req/headers/matches/many");
    get(&srv.uri())
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .await.unwrap()
        .assert_ok();
}

#[async_std::test]
async fn should_fail_when_one_of_header_regex_not_respected() {
    let srv = given("req/headers/matches/many");
    get(&srv.uri())
        .header("Content-Type", "app/json")
        .header("Accept", "application/json")
        .await.unwrap().assert_not_found();
    get(&srv.uri())
        .header("Content-Type", "application/json")
        .header("Accept", "app/json")
        .await.unwrap().assert_not_found();
    get(&srv.uri())
        .header("Content-Type", "app/json")
        .header("Accept", "app/json")
        .await.unwrap().assert_not_found();
    get(&srv.uri()).header("Content-Type", "application/json").await.unwrap().assert_not_found();
    get(&srv.uri()).header("Accept", "application/json").await.unwrap().assert_not_found();
    get(&srv.uri()).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn negative_should_map_request_when_header_matches() {
    let srv = given("req/headers/matches/negative");
    get(&srv.uri()).header("Content-Type", "application/json").await.unwrap().assert_ok();
    get(&srv.uri()).header("Content-Type", "any").await.unwrap().assert_ok();
}

#[async_std::test]
async fn negative_should_fail_when_absent() {
    let srv = given("req/headers/matches/negative");
    get(&srv.uri()).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn negative_should_fail_when_regex_not_respected() {
    let srv = given("req/headers/matches/negative");
    get(&srv.uri()).header("Content-Type", "text/plain").await.unwrap().assert_not_found();
}

#[async_std::test]
async fn negative_should_map_request_when_many_header_matches() {
    let srv = given("req/headers/matches/negative-many");
    get(&srv.uri())
        .header("Content-Type", "application/json")
        .header("X-Some", "application/json")
        .await.unwrap()
        .assert_ok();
}

// using 'X-Some' because surf adds by default a relaxed Accept header
#[async_std::test]
async fn negative_should_fail_when_one_of_header_regex_not_respected() {
    let srv = given("req/headers/matches/negative-many");
    get(&srv.uri())
        .header("Content-Type", "application/json")
        .header("X-Some", "text/plain")
        .await.unwrap()
        .assert_not_found();
    get(&srv.uri())
        .header("Content-Type", "text/plain")
        .header("X-Some", "application/json")
        .await.unwrap()
        .assert_not_found();
    get(&srv.uri())
        .header("Content-Type", "text/plain")
        .header("X-Some", "text/plain")
        .await.unwrap()
        .assert_not_found();
    get(&srv.uri()).header("Content-Type", "application/json").await.unwrap().assert_not_found();
    get(&srv.uri()).header("X-Some", "application/json").await.unwrap().assert_not_found();
    get(&srv.uri()).await.unwrap().assert_not_found();
}
