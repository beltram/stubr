use asserhttp::*;
use surf::get;

use stubr::Config;

use crate::utils::*;

#[async_std::test]
async fn should_verify_that_no_request_is_made_success() {
    let cfg = Config {
        verify: Some(true),
        ..Default::default()
    };
    let _srv = Stubr::start_with("tests/stubs/resp/verify/expect-0.json", cfg).await;
}

#[async_std::test]
#[should_panic]
async fn should_verify_that_no_request_is_made_panic() {
    let cfg = Config {
        verify: Some(true),
        ..Default::default()
    };
    let srv = Stubr::start_with("tests/stubs/resp/verify/expect-0.json", cfg).await;
    get(&srv.uri()).await.expect_status_ok();
}

#[async_std::test]
#[should_panic]
async fn should_verify_that_a_request_is_made_panic() {
    let cfg = Config {
        verify: Some(true),
        ..Default::default()
    };
    let _srv = Stubr::start_with("tests/stubs/resp/verify/expect-1.json", cfg).await;
}

#[async_std::test]
#[should_panic]
async fn should_verify_that_two_requests_are_made_panic() {
    let cfg = Config {
        verify: Some(true),
        ..Default::default()
    };
    let srv = Stubr::start_with("tests/stubs/resp/verify/expect-2.json", cfg).await;
    get(&srv.uri()).await.expect_status_ok();
}

#[async_std::test]
async fn should_verify_that_two_requests_are_made_success() {
    let cfg = Config {
        verify: Some(true),
        ..Default::default()
    };
    let srv = Stubr::start_with("tests/stubs/resp/verify/expect-2.json", cfg).await;
    get(&srv.uri()).await.expect_status_ok();
    get(&srv.uri()).await.expect_status_ok();
}
