use asserhttp::*;
use surf::get;

#[tokio::test]
#[stubr::mock]
async fn should_be_executable_in_a_tokio_runtime() {
    get(stubr.uri()).await.expect_status_ok();
}

#[tokio::test(flavor = "multi_thread")]
#[stubr::mock]
async fn should_be_executable_in_a_tokio_multi_threaded_runtime() {
    get(stubr.uri()).await.expect_status_ok();
}
