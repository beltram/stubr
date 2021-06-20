use asserhttp::*;
use surf::get;

#[async_std::test]
#[stubr::mock("req/auth")]
async fn should_support_basic_auth() {
    get(stubr.uri()).header("Authorization", "Basic dXNlcjpwYXNz").await
        .expect_status_ok();
}

#[async_std::test]
#[stubr::mock("req/auth")]
async fn basic_auth_should_fail_when_username_incorrect() {
    get(stubr.uri()).header("Authorization", "Basic bm90LXVzZXI6cGFzcw==").await
        .expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/auth")]
async fn basic_auth_should_fail_when_password_incorrect() {
    get(stubr.uri()).header("Authorization", "Basic dXNlcjpub3QtcGFzcw==").await
        .expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/auth")]
async fn basic_auth_should_fail_when_authorization_header_missing() {
    get(stubr.uri()).await
        .expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/auth")]
async fn basic_auth_should_fail_when_basic_prefix_missing() {
    get(stubr.uri()).header("Authorization", "dXNlcjpwYXNz").await
        .expect_status_not_found();
}

#[async_std::test]
#[stubr::mock("req/auth")]
async fn basic_auth_should_fail_when_double_colon_missing() {
    get(stubr.uri()).header("Authorization", "Basic dXNlcnBhc3M=").await
        .expect_status_not_found();
}