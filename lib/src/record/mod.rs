use http_types::{Request as HttpRequest, Response as HttpResponse};

use config::RecordConfig;

#[cfg(feature = "record-standalone")]
pub mod standalone;

pub mod client;
pub mod config;
pub mod core;
mod mapping;
pub mod record_client;
mod writer;

#[derive(Debug, Clone)]
pub struct RecordedRequest(pub HttpRequest);

#[derive(Debug, Clone)]
pub struct RecordedResponse(pub HttpResponse);

/// Intermediate representation of the request/response to be mapped further on into a stub
#[derive(Debug, Clone)]
pub struct RecordedExchange(pub RecordedRequest, pub RecordedResponse);

impl RecordedExchange {
    const DEFAULT_HOST: &'static str = "localhost";

    pub fn req(&self) -> &HttpRequest {
        &self.0 .0
    }
    pub fn resp(&self) -> &HttpResponse {
        &self.1 .0
    }
    pub fn host(&self) -> &str {
        self.req().host().unwrap_or(Self::DEFAULT_HOST)
    }
}

type RecordInput<'a> = (&'a mut RecordedExchange, &'a RecordConfig);
