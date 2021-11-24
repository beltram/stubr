use std::time::Duration;

use asserhttp::AsserhttpStatus;
use surf::{get, post};

use utils::StubrCli;

mod utils;

// #[async_std::test]
// TODO: too flaky
#[allow(dead_code)]
async fn should_serve_stubs_under_dir() {
    let stubr = StubrCli::new(&["tests/stubs"]);
    std::thread::sleep(Duration::from_millis(500));
    get(&stubr.addr).await.expect_status_success();
    post(&stubr.addr).await.expect_status_client_error();
}

// #[async_std::test]
// TODO: too flaky
#[allow(dead_code)]
async fn should_serve_stubs_under_root_dir() {
    let stubr = StubrCli::new(&["--root-dir", "tests/stubs"]);
    std::thread::sleep(Duration::from_millis(500));
    post(&stubr.addr).await.expect_status_success();
    get(&stubr.addr).await.expect_status_client_error();
}

// #[async_std::test]
// TODO: too flaky
#[allow(dead_code)]
async fn should_start_even_without_stubs() {
    let stubr = StubrCli::new(&[]);
    std::thread::sleep(Duration::from_millis(500));
    get(format!("{}/healtz", stubr.addr)).await.expect_status_success();
}