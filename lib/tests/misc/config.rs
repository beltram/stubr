use asserhttp::*;
use surf::get;

use stubr::Config;

use crate::utils::*;

#[async_std::test]
async fn should_start_server_on_dedicated_port() {
    let cfg = Config { port: Some(59_999), ..Default::default() };
    let srv = Stubr::start_with("tests/stubs/ping.json", cfg).await;
    let expected_uri = "http://127.0.0.1:59999";
    assert_eq!(srv.uri().as_str(), expected_uri);
    get(expected_uri).await.expect_status_ok();
}

#[async_std::test]
async fn should_start_server_in_a_blocking_way_with_some_configuration() {
    let cfg = Config { port: Some(59_998), ..Default::default() };
    let srv = Stubr::start_blocking_with("tests/stubs/ping.json", cfg);
    let expected_uri = "http://127.0.0.1:59998";
    assert_eq!(srv.uri().as_str(), expected_uri);
    get(expected_uri).await.expect_status_ok();
}