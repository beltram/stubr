use std::time::Duration;

use asserhttp::*;
use async_std::{io, task};
use surf::get;

use crate::utils::*;

#[async_std::test]
async fn should_timeout_with_delay_of_2_seconds() {
    let srv = given("resp/delay/2-seconds");
    let timeout = task::block_on(io::timeout(Duration::from_secs(1), async {
        get(&srv.uri()).await.expect_status_ok();
        Ok(())
    }));
    assert!(timeout.is_err());
}

#[async_std::test]
async fn should_not_timeout_with_delay_of_2_seconds() {
    let srv = given("resp/delay/2-seconds");
    let timeout = task::block_on(io::timeout(Duration::from_secs(3), async {
        get(&srv.uri()).await.expect_status_ok();
        Ok(())
    }));
    assert!(timeout.is_ok());
}

#[async_std::test]
async fn should_not_timeout_with_no_delay() {
    let srv = given("resp/delay/no-delay");
    let timeout = task::block_on(io::timeout(Duration::from_millis(100), async {
        get(&srv.uri()).await.expect_status_ok();
        Ok(())
    }));
    assert!(timeout.is_ok());
}