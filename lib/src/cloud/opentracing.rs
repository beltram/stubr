use std::str::FromStr;

use http_types::headers::HeaderName;
use wiremock::{Request, ResponseTemplate};

pub struct OpenTracing<'a>(pub &'a Request);

impl<'a> OpenTracing<'a> {
    const TRACE_ID_KEY: &'static str = "x-b3-traceid";
    const SPAN_ID_KEY: &'static str = "x-b3-spanid";
    const PARENT_SPAN_ID_KEY: &'static str = "x-b3-parentspanid";
    const SAMPLED_KEY: &'static str = "x-b3-sampled";
    const B3_KEY: &'static str = "b3";
    const OPEN_TRACING_HEADERS: [&'static str; 5] = [Self::TRACE_ID_KEY, Self::SPAN_ID_KEY, Self::PARENT_SPAN_ID_KEY, Self::SAMPLED_KEY, Self::B3_KEY];

    pub fn add_opentracing_header(&'a self, mut resp: ResponseTemplate, stub_headers: Option<impl Iterator<Item=&'a str>>) -> ResponseTemplate {
        if let Some(h) = stub_headers {
            for (k, v) in self.mixed_tracing_headers(h) {
                resp = resp.insert_header(k, v);
            }
        } else {
            for (k, v) in self.tracing_headers() {
                resp = resp.insert_header(k, v);
            }
        }
        resp
    }

    /// considering headers defined in stubs
    fn mixed_tracing_headers(&'a self, stub_headers: impl Iterator<Item=&'a str>) -> impl Iterator<Item=(&'a str, &'a str)> {
        stub_headers
            .filter(|it| !Self::OPEN_TRACING_HEADERS.contains(it))
            .filter_map(|k| self.req_header(k).map(|v| (k, v)))
    }

    fn tracing_headers(&'a self) -> impl Iterator<Item=(&'a str, &'a str)> {
        Self::OPEN_TRACING_HEADERS.into_iter()
            .filter_map(|k| self.req_header(k).map(|v| (k, v)))
    }

    fn req_header(&'a self, key: &'a str) -> Option<&'a str> {
        HeaderName::from_str(key).ok().as_ref()
            .and_then(|k| self.0.headers.get(k))
            .map(|v| v.as_str())
    }
}