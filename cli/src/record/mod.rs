use std::time::Duration;

use config::CliRecordConfig;
use stubr::Stubr;

pub mod config;

pub struct Record;

impl Record {
    const SLEEP_DURATION: Duration = Duration::from_millis(1000);

    pub async fn record(config: CliRecordConfig) {
        Stubr::record_with(config.into());
        loop { async_std::task::sleep(Self::SLEEP_DURATION).await; }
    }
}