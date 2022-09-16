use asserhttp::*;
use surf::get;

use stubr::Config;

use crate::utils::*;

#[async_std::test]
async fn should_succeed_when_no_request_made_and_none_expected() {
    let cfg = Config {
        verify: Some(true),
        ..Default::default()
    };
    let _srv = Stubr::start_with("tests/stubs/resp/verify/expect-0.json", cfg).await;
}

#[async_std::test]
#[should_panic]
async fn should_fail_when_a_request_is_made_but_none_expected() {
    let cfg = Config {
        verify: Some(true),
        ..Default::default()
    };
    let srv = Stubr::start_with("tests/stubs/resp/verify/expect-0.json", cfg).await;
    get(&srv.uri()).await.expect_status_ok();
}

#[async_std::test]
#[stubr::mock(full_path = "tests/stubs/resp/verify/expect-1.json", verify = true)]
#[should_panic]
async fn should_fail_when_no_request_is_made_but_1_expected() {
    // no request made
}

#[async_std::test]
#[stubr::mock(full_path = "tests/stubs/resp/verify/expect-1.json", verify = true)]
async fn should_succeed_when_1_request_made_and_1_expected() {
    get(&stubr.uri()).await.expect_status_ok();
}
