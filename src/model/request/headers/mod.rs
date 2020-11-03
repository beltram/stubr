use std::convert::TryFrom;

use itertools::Itertools;
use serde::Deserialize;
use serde_json::{Map, Value};
use wiremock::matchers::HeaderExactMatcher;
use wiremock::MockBuilder;

use case_insensitive::HeaderCaseInsensitiveMatcher;
use contains::HeaderContainsMatcher;
use value::HeaderValue;

use super::super::request::MockRegistrable;

pub mod value;
pub mod case_insensitive;
pub mod exact;
pub mod contains;

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
        for contains in Vec::<HeaderContainsMatcher>::from(self) {
            mock = mock.and(contains);
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
    fn is_exact_match(&self) -> bool {
        self.is_equal_to() && !self.is_case_insensitive() && !self.is_contains()
    }

    fn is_equal_to(&self) -> bool {
        self.value.as_ref()
            .and_then(|v| v.equal_to.as_ref())
            .map(|it| !it.is_empty())
            .unwrap_or_default()
    }

    fn is_case_insensitive(&self) -> bool {
        self.value.as_ref()
            .and_then(|v| v.case_insensitive)
            .unwrap_or_default()
    }

    fn is_contains(&self) -> bool {
        self.value.as_ref()
            .and_then(|v| v.contains.as_ref())
            .map(|it| !it.is_empty())
            .unwrap_or_default()
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
