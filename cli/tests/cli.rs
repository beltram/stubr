use surf::{get, post};

use utils::StubrCli;

mod utils;

#[async_std::test]
async fn should_serve_stubs_under_dir() {
    let stubr = StubrCli::new(&["tests/stubs"]);
    assert!(get(&stubr.addr).await.unwrap().status().is_success());
    assert!(post(&stubr.addr).await.unwrap().status().is_client_error());
}

#[async_std::test]
async fn should_serve_stubs_under_root_dir() {
    let stubr = StubrCli::new(&["--root-dir", "tests/stubs"]);
    assert!(post(&stubr.addr).await.unwrap().status().is_success());
    assert!(get(&stubr.addr).await.unwrap().status().is_client_error());
}