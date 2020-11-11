use std::convert::TryFrom;

use itertools::Itertools;
use serde::Deserialize;
use serde_json::{Map, Value};
use wiremock::matchers::HeaderExactMatcher;
use wiremock::MockBuilder;

use case::HeaderCaseInsensitiveMatcher;
use contains::HeaderContainsMatcher;

use super::matcher::RequestMatcherDto;
use super::super::request::MockRegistrable;

pub mod case;
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
    fn get_headers(&self) -> Vec<RequestMatcherDto> {
        self.headers.as_ref()
            .map(|h| h.iter().map(RequestMatcherDto::try_from))
            .map(|it| it.flatten().collect_vec())
            .unwrap_or_default()
    }
}