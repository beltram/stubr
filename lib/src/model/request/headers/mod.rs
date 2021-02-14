use std::convert::TryFrom;

use itertools::Itertools;
use serde::Deserialize;
use serde_json::{Map, Value};
use wiremock::{matchers::HeaderExactMatcher, MockBuilder};

use absent::HeaderAbsentMatcher;
use case::HeaderCaseInsensitiveMatcher;
use contains::HeaderContainsMatcher;
use matches::HeaderRegexMatcher;

use super::{matcher::RequestMatcherDto, super::request::MockRegistrable};

mod case;
mod exact;
mod contains;
mod matches;
mod absent;

#[derive(Deserialize, Debug, Default)]
pub struct HttpReqHeadersDto {
    // matches all request http headers
    headers: Option<Map<String, Value>>,
}

impl MockRegistrable for HttpReqHeadersDto {
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
        for matches in Vec::<HeaderRegexMatcher>::from(self) {
            mock = mock.and(matches);
        }
        for absent in Vec::<HeaderAbsentMatcher>::from(self) {
            mock = mock.and(absent);
        }
        mock
    }
}

impl HttpReqHeadersDto {
    fn get_headers(&self) -> Vec<RequestMatcherDto> {
        self.headers.as_ref()
            .map(|h| h.iter().map(RequestMatcherDto::try_from))
            .map(|it| it.flatten().collect_vec())
            .unwrap_or_default()
    }
}