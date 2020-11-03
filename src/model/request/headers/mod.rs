use std::convert::TryFrom;

use itertools::Itertools;
use serde::Deserialize;
use serde_json::{Map, Value};
use wiremock::matchers::HeaderExactMatcher;
use wiremock::MockBuilder;

use case_insensitive::HeaderCaseInsensitiveMatcher;
use value::HeaderValue;

use super::super::request::MockRegistrable;

pub mod value;
pub mod case_insensitive;
pub mod exact;

#[derive(Deserialize, Debug, Default)]
pub struct HttpReqHeaders {
    // matches all request http headers
    headers: Option<Map<String, Value>>,
}

impl MockRegistrable for HttpReqHeaders {
    fn register(&self, mut mock: MockBuilder) -> MockBuilder {
        for exact in Vec::<HeaderExactMatcher>::from(self) {
            mock = mock.and(exact);
        }
        for case_insensitive in Vec::<HeaderCaseInsensitiveMatcher>::from(self) {
            mock = mock.and(case_insensitive);
        }
        mock
    }
}

impl HttpReqHeaders {
    fn get_headers(&self) -> Vec<Header> {
        self.headers.as_ref()
            .map(|h| h.iter().map(Header::try_from))
            .map(|it| it.flatten().collect_vec())
            .unwrap_or_default()
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct Header {
    // header key e.g. 'Content-Type'
    pub key: String,
    pub value: Option<HeaderValue>,
}

impl Header {
    fn is_case_insensitive(&self) -> bool {
        self.value.as_ref()
            .and_then(|v| v.case_insensitive)
            .map_or(false, |case| case)
    }
}

impl TryFrom<(&String, &Value)> for Header {
    type Error = anyhow::Error;

    fn try_from((k, v): (&String, &Value)) -> anyhow::Result<Self> {
        Ok(Self {
            key: k.to_owned(),
            value: serde_json::from_value(v.to_owned()).ok(),
        })
    }
}
