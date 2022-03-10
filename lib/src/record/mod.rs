use http_types::{Request as HttpRequest, Response as HttpResponse};

use config::RecordConfig;

#[cfg(feature = "record-standalone")]
pub mod standalone;

mod mapping;
pub mod config;
pub mod client;
mod writer;
pub mod core;
pub mod record_client;

#[derive(Debug, Clone)]
pub struct RecordedRequest(pub HttpRequest);

#[derive(Debug, Clone)]
pub struct RecordedResponse(pub HttpResponse);

/// Intermediate representation of the request/response to be mapped further on into a stub
#[derive(Debug, Clone)]
pub struct RecordedExchange(pub RecordedRequest, pub RecordedResponse);

impl RecordedExchange {
    const DEFAULT_HOST: &'static str = "localhost";

    pub fn req(&self) -> &HttpRequest { &self.0.0 }
    pub fn resp(&self) -> &HttpResponse { &self.1.0 }
    pub fn host(&self) -> &str { self.req().host().unwrap_or(Self::DEFAULT_HOST) }
}

type RecordInput<'a> = (&'a mut RecordedExchange, &'a RecordConfig);