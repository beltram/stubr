use std::net::SocketAddr;

use log::{error, info};
use tokio::sync::mpsc::Sender;

use logger::RecordLogger;
use proxy::Proxy;

use super::config::RecordConfig;
use super::super::model::JsonStub;
use super::writer::StubWriter;

pub mod http;
pub mod proxy;
pub mod logger;
pub mod warp_exchange;
pub mod port;

#[cfg(feature = "record-standalone")]
pub struct StubrRecord {
    addr: SocketAddr,
    tx: Sender<String>,
}

#[cfg(feature = "record-standalone")]
impl StubrRecord {
    /// Get recorder base uri.
    /// Use this to configure your http client proxy configuration.
    pub fn uri(&self) -> String {
        format!("http://{}", self.addr)
    }

    pub(crate) fn record(config: RecordConfig) -> Self {
        let (addr, tx) = Proxy::run(config, |(ex, cfg)| {
            let host = format!("http://{}", ex.host());
            let method = ex.req().method().to_string();
            let url = ex.req().url().clone();
            let status: u16 = ex.resp().status().into();
            let stub = JsonStub::from((ex, cfg));
            let writer = StubWriter { stub };
            writer.write(&host, cfg.output.as_ref())
                .map(|f| RecordLogger::success(f, status, &method, &url))
                .unwrap_or_else(|e| RecordLogger::error(e, status, &method, &url));
        });
        Self { addr, tx }
    }

    #[cfg(feature = "record-isahc")]
    pub fn isahc_client(&self) -> isahc::HttpClient {
        crate::isahc_client(self.uri())
    }

    #[cfg(feature = "record-reqwest")]
    pub fn reqwest_client(&self) -> reqwest::Client {
        crate::reqwest_client(self.uri())
    }
}

#[cfg(feature = "record-standalone")]
impl Drop for StubrRecord {
    fn drop(&mut self) {
        async_std::task::block_on(async {
            match self.tx.send(String::new()).await {
                Ok(_) => info!("Stopping stubr recorder on {}", self.addr),
                Err(e) => error!("Failed stopping stubr recorder on {} because {:?}", self.addr, e)
            }
        });
    }
}