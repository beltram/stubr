use std::convert::TryFrom;
use std::hash::{Hash, Hasher};

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use wiremock::{matchers::HeaderExactMatcher, MockBuilder};

use absent::HeaderAbsentMatcher;
use case::HeaderCaseInsensitiveMatcher;
use contains::HeaderContainsMatcher;
use matches::HeaderRegexMatcher;

use super::{matcher::RequestMatcherStub, super::request::MockRegistrable};

mod case;
mod exact;
mod contains;
mod matches;
mod absent;

#[derive(Serialize, Deserialize, Debug, Default, Eq)]
pub struct HttpReqHeadersStub {
    // matches all request http headers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Map<String, Value>>,
}

impl MockRegistrable for HttpReqHeadersStub {
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

impl HttpReqHeadersStub {
    fn get_headers(&self) -> Vec<RequestMatcherStub> {
        self.headers.as_ref()
            .map(|h| h.iter().map(RequestMatcherStub::try_from))
            .map(|it| it.flatten().collect_vec())
            .unwrap_or_default()
    }
}

impl PartialEq for HttpReqHeadersStub {
    fn eq(&self, other: &Self) -> bool {
        self.headers.as_ref().eq(&other.headers.as_ref())
    }
}

impl Hash for HttpReqHeadersStub {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(headers) = self.headers.as_ref() {
            headers.iter()
                .for_each(|(k, v)| {
                    k.hash(state);
                    v.is_string().hash(state);
                })
        }
    }
}