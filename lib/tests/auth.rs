use surf::get;

use crate::utils::*;

mod utils;

#[async_std::test]
async fn should_support_basic_auth() {
    let srv = given("req/auth/basic");
    get(&srv.url()).header("Authorization", "Basic dXNlcjpwYXNz").await.unwrap()
        .assert_ok();
}

#[async_std::test]
async fn basic_auth_should_fail_when_username_incorrect() {
    let srv = given("req/auth/basic");
    get(&srv.url()).header("Authorization", "Basic bm90LXVzZXI6cGFzcw==").await.unwrap()
        .assert_not_found();
}

#[async_std::test]
async fn basic_auth_should_fail_when_password_incorrect() {
    let srv = given("req/auth/basic");
    get(&srv.url()).header("Authorization", "Basic dXNlcjpub3QtcGFzcw==").await.unwrap()
        .assert_not_found();
}

#[async_std::test]
async fn basic_auth_should_fail_when_authorization_header_missing() {
    let srv = given("req/auth/basic");
    get(&srv.url()).await.unwrap()
        .assert_not_found();
}

#[async_std::test]
async fn basic_auth_should_fail_when_basic_prefix_missing() {
    let srv = given("req/auth/basic");
    get(&srv.url()).header("Authorization", "dXNlcjpwYXNz").await.unwrap()
        .assert_not_found();
}

#[async_std::test]
async fn basic_auth_should_fail_when_double_colon_missing() {
    let srv = given("req/auth/basic");
    get(&srv.url()).header("Authorization", "Basic dXNlcnBhc3M=").await.unwrap()
        .assert_not_found();
}