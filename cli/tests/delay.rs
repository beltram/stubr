use std::time::Duration;

use async_std::{io, task};
use surf::get;

use utils::StubrCli;

mod utils;

#[async_std::test]
async fn should_timeout_with_global_delay_of_2_seconds() {
    let stubr = StubrCli::new(&["tests/stubs", "--delay", "2s"]);
    let timeout = task::block_on(io::timeout(Duration::from_secs(1), async {
        get(&stubr.addr).await.unwrap().status().is_success();
        Ok(())
    }));
    assert!(timeout.is_err());
}

#[async_std::test]
async fn should_not_timeout_with_global_delay_of_2_seconds() {
    let stubr = StubrCli::new(&["tests/stubs", "--delay", "2s"]);
    let timeout = task::block_on(io::timeout(Duration::from_secs(3), async {
        get(&stubr.addr).await.unwrap().status().is_success();
        Ok(())
    }));
    assert!(timeout.is_ok());
}

#[async_std::test]
async fn should_timeout_with_latency_of_2_seconds() {
    let stubr = StubrCli::new(&["tests/stubs", "--latency", "2s"]);
    let timeout = task::block_on(io::timeout(Duration::from_secs(1), async {
        get(&stubr.addr).await.unwrap().status().is_success();
        Ok(())
    }));
    assert!(timeout.is_err());
}

#[async_std::test]
async fn should_not_timeout_with_latency_of_2_seconds() {
    let stubr = StubrCli::new(&["tests/stubs", "--latency", "2s"]);
    let timeout = task::block_on(io::timeout(Duration::from_secs(3), async {
        get(&stubr.addr).await.unwrap().status().is_success();
        Ok(())
    }));
    assert!(timeout.is_ok());
}