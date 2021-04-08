use std::str::FromStr;

use http_types::headers::HeaderName;
use wiremock::{Request, ResponseTemplate};

pub struct OpenTracing<'a>(pub &'a Request, pub Vec<&'a str>);

impl<'a> OpenTracing<'a> {
    const TRACE_ID_KEY: &'static str = "x-b3-traceid";
    const SPAN_ID_KEY: &'static str = "x-b3-spanid";
    const PARENT_SPAN_ID_KEY: &'static str = "x-b3-parentspanid";
    const SAMPLED_KEY: &'static str = "x-b3-sampled";
    const B3_KEY: &'static str = "b3";

    pub fn add_opentracing_header(&self, mut resp: ResponseTemplate) -> ResponseTemplate {
        if let Some(trace_id) = self.get_header(Self::TRACE_ID_KEY) {
            resp = resp.insert_header(Self::TRACE_ID_KEY, trace_id);
        }
        if let Some(span_id) = self.get_header(Self::SPAN_ID_KEY) {
            resp = resp.insert_header(Self::SPAN_ID_KEY, span_id);
        }
        if let Some(parent_span_id) = self.get_header(Self::PARENT_SPAN_ID_KEY) {
            resp = resp.insert_header(Self::PARENT_SPAN_ID_KEY, parent_span_id);
        }
        if let Some(sampled) = self.get_header(Self::SAMPLED_KEY) {
            resp = resp.insert_header(Self::SAMPLED_KEY, sampled);
        }
        if let Some(b3) = self.get_header(Self::B3_KEY) {
            resp = resp.insert_header(Self::B3_KEY, b3);
        }
        resp
    }

    fn get_header(&self, key: &str) -> Option<&str> {
        if !self.is_header_already_defined(key) {
            HeaderName::from_str(key).ok().as_ref()
                .and_then(|k| self.0.headers.get(k))
                .map(|it| it.as_str())
        } else { None }
    }

    fn is_header_already_defined(&self, key: &str) -> bool {
        self.1.contains(&key)
    }
}