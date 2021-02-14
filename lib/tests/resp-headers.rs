use surf::get;

use crate::utils::*;

mod utils;

#[async_std::test]
async fn should_return_single_response_header() {
    let srv = given("resp/headers/single");
    get(&srv.uri()).await.unwrap()
        .assert_ok()
        .assert_header("X-Header-1", "1");
}

#[async_std::test]
async fn should_return_many_response_header() {
    let srv = given("resp/headers/many");
    get(&srv.uri()).await.unwrap()
        .assert_ok()
        .assert_header("X-Header-1", "1")
        .assert_header("X-Header-2", "2");
}

#[async_std::test]
async fn should_not_return_header_when_absent() {
    let srv = given("resp/headers/none");
    get(&srv.uri()).await.unwrap()
        .assert_ok()
        .assert_no_header("X-Header-1");
}

#[async_std::test]
async fn user_defined_server_header_should_have_precedence_over_default_one() {
    let srv = given("resp/headers/server");
    get(&srv.uri()).await.unwrap()
        .assert_ok()
        .assert_header("Server", "my-app");
}