use std::convert::TryFrom;
use std::hash::{Hash, Hasher};

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use wiremock::{matchers::QueryParamExactMatcher, MockBuilder};

use absent::QueryAbsentMatcher;
use case::QueryCaseInsensitiveMatcher;
use contains::QueryContainsMatcher;
use matches::QueryRegexMatcher;

use super::{matcher::RequestMatcherStub, super::request::MockRegistrable};

mod exact;
mod case;
mod contains;
mod matches;
mod absent;

#[derive(Serialize, Deserialize, Debug, Default, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HttpQueryParamsStub {
    // matches all request http headers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_parameters: Option<Map<String, Value>>,
}

impl MockRegistrable for HttpQueryParamsStub {
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

impl HttpQueryParamsStub {
    pub fn get_queries(&self) -> Vec<RequestMatcherStub> {
        self.query_parameters.as_ref()
            .map(|h| h.iter().map(RequestMatcherStub::try_from))
            .map(|it| it.flatten().collect_vec())
            .unwrap_or_default()
    }
}

impl PartialEq for HttpQueryParamsStub {
    fn eq(&self, other: &Self) -> bool {
        self.query_parameters.as_ref().eq(&other.query_parameters.as_ref())
    }
}

impl Hash for HttpQueryParamsStub {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(queries) = self.query_parameters.as_ref() {
            queries.iter()
                .for_each(|(k, v)| {
                    k.hash(state);
                    v.to_string().hash(state);
                })
        }
    }
}