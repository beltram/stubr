use std::net::SocketAddr;

#[cfg(feature = "record-isahc")]
use isahc::HttpClient as IsahcClient;
use log::{error, info};
#[cfg(feature = "record-reqwest")]
use reqwest::Client as ReqwestClient;
use tokio::sync::mpsc::Sender;

use config::RecordConfig;
use logger::RecordLogger;
use proxy::Proxy;
use writer::StubWriter;

#[cfg(feature = "record-isahc")]
use crate::isahc_client;
#[cfg(feature = "record-reqwest")]
use crate::reqwest_client;

use super::model::JsonStub;

use self::http::RecordedExchange;

mod http;
mod proxy;
mod mapping;
pub mod config;
mod warp_exchange;
pub mod client;
mod port;
mod writer;
mod logger;
pub mod core;
pub mod record_client;

type RecordInput<'a> = (&'a mut RecordedExchange, &'a RecordConfig);

pub struct StubrRecord {
    addr: SocketAddr,
    tx: Sender<String>,
}

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
    pub fn isahc_client(&self) -> IsahcClient {
        isahc_client(self.uri())
    }

    #[cfg(feature = "record-reqwest")]
    pub fn reqwest_client(&self) -> ReqwestClient {
        reqwest_client(self.uri())
    }
}

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