use std::{convert::TryFrom, hash::{Hash, Hasher}};

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
        if let Ok(matchers) = Vec::<QueryParamExactMatcher>::try_from(self) {
            for exact in matchers { mock = mock.and(exact); }
        }
        if let Ok(matchers) = Vec::<QueryCaseInsensitiveMatcher>::try_from(self) {
            for case in matchers { mock = mock.and(case); }
        }
        if let Ok(matchers) = Vec::<QueryContainsMatcher>::try_from(self) {
            for contains in matchers { mock = mock.and(contains); }
        }
        if let Ok(matchers) = Vec::<QueryRegexMatcher>::try_from(self) {
            for regex in matchers { mock = mock.and(regex); }
        }
        if let Ok(matchers) = Vec::<QueryAbsentMatcher>::try_from(self) {
            for absent in matchers { mock = mock.and(absent); }
        }
        mock
    }
}

impl HttpQueryParamsStub {
    pub fn get_queries(&self) -> Option<impl Iterator<Item=RequestMatcherStub> + '_> {
        self.query_parameters.as_ref()
            .map(|h| h.iter().filter_map(|it| RequestMatcherStub::try_from(it).ok()))
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