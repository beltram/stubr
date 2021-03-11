use std::time::Duration;

use async_std::{io, task};
use surf::get;

use stubr::Config;

use crate::utils::*;

mod utils;

#[async_std::test]
async fn should_start_server_on_dedicated_port() {
    let cfg = Config { port: Some(59_999), ..Default::default() };
    let srv = Stubr::start_with("tests/stubs/ping.json", cfg).await;
    let expected_uri = "http://127.0.0.1:59999";
    assert_eq!(srv.uri().as_str(), expected_uri);
    get(expected_uri).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_start_server_in_a_blocking_way_with_some_configuration() {
    let cfg = Config { port: Some(59_998), ..Default::default() };
    let srv = Stubr::start_blocking_with("tests/stubs/ping.json", cfg);
    let expected_uri = "http://127.0.0.1:59998";
    assert_eq!(srv.uri().as_str(), expected_uri);
    get(expected_uri).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_timeout_with_global_delay_of_2_seconds() {
    let cfg = Config { global_delay: Some(2000), ..Default::default() };
    let srv = Stubr::start_with("tests/stubs/ping.json", cfg).await;
    let timeout = task::block_on(io::timeout(Duration::from_secs(1), async {
        get(&srv.url()).await.unwrap().assert_ok();
        Ok(())
    }));
    assert!(timeout.is_err())
}

#[async_std::test]
async fn should_not_timeout_with_global_delay_of_2_seconds() {
    let cfg = Config { global_delay: Some(2000), ..Default::default() };
    let srv = Stubr::start_with("tests/stubs/ping.json", cfg).await;
    let timeout = task::block_on(io::timeout(Duration::from_secs(3), async {
        get(&srv.url()).await.unwrap().assert_ok();
        Ok(())
    }));
    assert!(timeout.is_ok())
}