use std::net::SocketAddr;

#[cfg(feature = "test-isahc")]
use isahc::HttpClient as IsahcHttpClient;

use config::RecordConfig;
use http::RecordedExchange;
use logger::RecordLogger;
use proxy::Proxy;
use writer::StubWriter;

#[cfg(feature = "test-isahc")]
use crate::isahc_client;

use super::model::JsonStub;

mod http;
mod proxy;
mod mapping;
pub mod config;
mod warp_exchange;
pub mod test;
mod port;
mod writer;
mod logger;

type RecordInput<'a> = (&'a mut RecordedExchange, RecordConfig);

pub struct StubrRecord {
    addr: SocketAddr,
}

impl StubrRecord {
    /// Get recorder base uri.
    /// Use this to configure your http client proxy configuration.
    pub fn uri(&self) -> String {
        format!("http://{}", self.addr)
    }

    pub(crate) fn record(config: RecordConfig) -> Self {
        let addr = Proxy::run(config, |(ex, cfg)| {
            let host = format!("http://{}", ex.host());
            let method = ex.req().method().to_string();
            let url = ex.req().url().clone();
            let status: u16 = ex.resp().status().into();
            let stub = JsonStub::from((ex, cfg.clone()));
            let writer = StubWriter { stub };
            writer.write(&host, cfg.output)
                .map(|f| RecordLogger::success(f, status, &method, &url))
                .unwrap_or_else(|e| RecordLogger::error(e, status, &method, &url));
        });
        Self { addr }
    }

    #[cfg(feature = "test-isahc")]
    pub fn isahc_client(&self) -> IsahcHttpClient {
        isahc_client(self.uri())
    }
}