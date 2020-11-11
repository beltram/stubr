use std::convert::TryFrom;

use itertools::Itertools;
use serde::Deserialize;
use serde_json::{Map, Value};
use wiremock::matchers::QueryParamExactMatcher;
use wiremock::MockBuilder;

use case::QueryCaseInsensitiveMatcher;
use contains::QueryContainsMatcher;
use value::QueryValue;

use super::super::request::MockRegistrable;

mod value;
mod exact;
mod case;
mod contains;

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct HttpQueryParams {
    // matches all request http headers
    query_parameters: Option<Map<String, Value>>,
}

impl MockRegistrable for HttpQueryParams {
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
        mock
    }
}

impl HttpQueryParams {
    fn get_queries(&self) -> Vec<Query> {
        self.query_parameters.as_ref()
            .map(|h| h.iter().map(Query::try_from))
            .map(|it| it.flatten().collect_vec())
            .unwrap_or_default()
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct Query {
    // query key e.g. 'age='
    pub key: String,
    pub value: Option<QueryValue>,
}

impl Query {
    fn is_exact_match(&self) -> bool {
        self.is_equal_to() && !self.is_case_insensitive() && !self.is_contains()
    }

    fn is_equal_to(&self) -> bool {
        self.value.as_ref()
            .and_then(|v| v.equal_to.as_ref())
            .is_some()
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

    fn equal_to_as_str(&self) -> Option<String> {
        self.value.as_ref()
            .and_then(|it| it.equal_to.as_ref())
            .and_then(|v| {
                v.as_str().map(|s| s.to_string())
                    .or_else(|| v.as_bool().map(|b| b.to_string()))
                    .or_else(|| v.as_i64().map(|i| i.to_string()))
            })
    }
}

impl TryFrom<(&String, &Value)> for Query {
    type Error = anyhow::Error;

    fn try_from((k, v): (&String, &Value)) -> anyhow::Result<Self> {
        Ok(Self {
            key: k.to_owned(),
            value: serde_json::from_value(v.to_owned()).ok(),
        })
    }
}