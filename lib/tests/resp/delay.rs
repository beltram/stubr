use std::time::Duration;

use asserhttp::*;
use async_std::{io, task};
use surf::get;

#[async_std::test]
#[stubr::mock("resp/delay/5-seconds.json")]
async fn should_timeout_with_delay_of_5_seconds() {
    let timeout = Duration::from_secs(1);
    let timeout = task::block_on(io::timeout(timeout, async {
        get(stubr.uri()).await.expect_status_ok();
        Ok(())
    }));
    assert!(timeout.is_err());
}

#[async_std::test]
#[stubr::mock("resp/delay/5-seconds.json")]
async fn should_not_timeout_with_delay_of_5_seconds() {
    let timeout = Duration::from_secs(30);
    let timeout = task::block_on(io::timeout(timeout, async {
        get(stubr.uri()).await.expect_status_ok();
        Ok(())
    }));
    assert!(timeout.is_ok());
}

#[async_std::test]
#[stubr::mock("resp/delay/no-delay.json")]
async fn should_not_timeout_with_no_delay() {
    let timeout = Duration::from_millis(100);
    let timeout = task::block_on(io::timeout(timeout, async {
        get(stubr.uri()).await.expect_status_ok();
        Ok(())
    }));
    assert!(timeout.is_ok());
}
