use std::convert::TryFrom;

use itertools::Itertools;
use serde::Deserialize;
use serde_json::{Map, Value};
use wiremock::matchers::QueryParamExactMatcher;
use wiremock::MockBuilder;

use absent::QueryAbsentMatcher;
use case::QueryCaseInsensitiveMatcher;
use contains::QueryContainsMatcher;
use matches::QueryRegexMatcher;

use super::matcher::RequestMatcherDto;
use super::super::request::MockRegistrable;

mod exact;
mod case;
mod contains;
mod matches;
mod absent;

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct HttpQueryParamsDto {
    // matches all request http headers
    query_parameters: Option<Map<String, Value>>,
}

impl MockRegistrable for HttpQueryParamsDto {
    fn register(&self, mut mock: MockBuilder) -> MockBuilder {
        for exact in Vec::<QueryParamExactMatcher>::from(self) {
            mock = mock.and(exact);
        }
        for case in Vec::<QueryCaseInsensitiveMatcher>::from(self) {
            mock = mock.and(case);
        }
        for contains in Vec::<QueryContainsMatcher>::from(self) {
            mock = mock.and(contains);
        }
        for regex in Vec::<QueryRegexMatcher>::from(self) {
            mock = mock.and(regex);
        }
        for absent in Vec::<QueryAbsentMatcher>::from(self) {
            mock = mock.and(absent);
        }
        mock
    }
}

impl HttpQueryParamsDto {
    fn get_queries(&self) -> Vec<RequestMatcherDto> {
        self.query_parameters.as_ref()
            .map(|h| h.iter().map(RequestMatcherDto::try_from))
            .map(|it| it.flatten().collect_vec())
            .unwrap_or_default()
    }
}