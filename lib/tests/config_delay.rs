use std::time::Duration;

use async_std::{io, task};
use surf::get;

use stubr::Config;

use crate::utils::*;

mod utils;

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

#[async_std::test]
async fn should_ignore_local_delay_defined_in_stub() {
    let cfg = Config { global_delay: Some(2000), ..Default::default() };
    let srv = Stubr::start_with("tests/stubs/resp/delay/2-seconds.json", cfg).await;
    let timeout = task::block_on(io::timeout(Duration::from_secs(3), async {
        get(&srv.url()).await.unwrap().assert_ok();
        Ok(())
    }));
    assert!(timeout.is_ok())
}