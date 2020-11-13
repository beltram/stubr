use surf::get;

use crate::utils::*;

mod utils;

#[async_std::test]
async fn should_map_text_response_body() {
    let srv = given("resp/body/text");
    get(&srv.uri()).await.unwrap()
        .assert_body("Hello World !")
        .assert_header("Content-Type", "text/plain");
}

#[async_std::test]
async fn should_map_blank_text_response_body() {
    let srv = given("resp/body/text-blank");
    get(&srv.uri()).await.unwrap()
        .assert_body(" ")
        .assert_header("Content-Type", "text/plain");
}

#[async_std::test]
async fn should_map_empty_text_response_body() {
    let srv = given("resp/body/text-empty");
    get(&srv.uri()).await.unwrap()
        .assert_body_empty()
        .assert_header("Content-Type", "text/plain");
}