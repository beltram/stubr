use std::time::Duration;

use asserhttp::*;
use async_std::{io, task};
use surf::get;

mod fixed {
    use super::*;

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
        let begin = std::time::Instant::now();
        get(stubr.uri()).await.expect_status_ok();
        let delta = std::time::Instant::now() - begin;
        assert!(delta.as_millis() < 500);
    }
}

mod lognormal {
    use super::*;

    // see [https://www.wolframalpha.com/input?i=lognormaldistribution%28log%285000%29%2C+1.0%29]
    #[async_std::test]
    #[stubr::mock("resp/delay/lognormal-m100-s01.json")]
    async fn should_timeout_with_rand_delay_of_100ms_with_01_sigma() {
        const ROUNDS: usize = 30;
        const DELTA: f64 = 15.0;
        const EXPECTED: f64 = 100.0;
        const LOWER: f64 = EXPECTED - DELTA;
        const UPPER: f64 = EXPECTED + DELTA;

        let mut sum = 0.0;
        for _ in 0..ROUNDS {
            let begin = std::time::Instant::now();
            get(stubr.uri()).await.expect_status_ok();
            let delta = std::time::Instant::now() - begin;
            sum += delta.as_millis() as f64;
        }
        let mean = sum / (ROUNDS as f64);
        assert!((mean > LOWER) && (mean < UPPER));
    }
}
