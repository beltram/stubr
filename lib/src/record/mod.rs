#[cfg(feature = "record")]
use std::net::SocketAddr;

use http_types::{Request as HttpRequest, Response as HttpResponse};
#[cfg(feature = "record-isahc")]
use isahc::HttpClient as IsahcClient;
#[cfg(feature = "record")]
use log::{error, info};
#[cfg(feature = "record-reqwest")]
use reqwest::Client as ReqwestClient;
#[cfg(feature = "record")]
use tokio::sync::mpsc::Sender;

use config::RecordConfig;
#[cfg(feature = "record")]
use logger::RecordLogger;
#[cfg(feature = "record")]
use proxy::Proxy;
#[cfg(feature = "record")]
use writer::StubWriter;

#[cfg(feature = "record-isahc")]
use crate::isahc_client;
#[cfg(feature = "record-reqwest")]
use crate::reqwest_client;

#[cfg(feature = "record")]
use super::model::JsonStub;

#[cfg(feature = "record")]
mod http;
#[cfg(feature = "record")]
mod proxy;
mod mapping;
pub mod config;
#[cfg(feature = "record")]
mod warp_exchange;
pub mod client;
#[cfg(feature = "record")]
mod port;
mod writer;
#[cfg(feature = "record")]
mod logger;
pub mod core;
pub mod record_client;

#[derive(Debug, Clone)]
pub struct RecordedRequest(pub HttpRequest);

#[derive(Debug, Clone)]
pub struct RecordedResponse(pub HttpResponse);

/// Intermediate representation of the request/response to be mapped further on into a stub
pub struct RecordedExchange(pub RecordedRequest, pub RecordedResponse);

impl RecordedExchange {
    const DEFAULT_HOST: &'static str = "localhost";

    pub fn req(&self) -> &HttpRequest { &self.0.0 }
    pub fn resp(&self) -> &HttpResponse { &self.1.0 }
    pub fn host(&self) -> &str { self.req().host().unwrap_or(Self::DEFAULT_HOST) }
}

type RecordInput<'a> = (&'a mut RecordedExchange, &'a RecordConfig);

#[cfg(feature = "record")]
pub struct StubrRecord {
    addr: SocketAddr,
    tx: Sender<String>,
}

#[cfg(feature = "record")]
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

#[cfg(feature = "record")]
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