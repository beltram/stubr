use std::convert::TryFrom;

use itertools::Itertools;
use wiremock::{Match, Request};

use super::{HttpQueryParams, Query};

fn query_case_insensitive(key: String, value: String) -> QueryCaseInsensitiveMatcher {
    QueryCaseInsensitiveMatcher(key, value)
}

pub struct QueryCaseInsensitiveMatcher(String, String);

impl Match for QueryCaseInsensitiveMatcher {
    fn matches(&self, request: &Request) -> bool {
        request.url.query_pairs()
            .find(|(k, _)| k == self.0.as_str())
            .map(|(_, v)| v.eq_ignore_ascii_case(self.1.as_str()))
            .unwrap_or_default()
    }
}

impl From<&HttpQueryParams> for Vec<QueryCaseInsensitiveMatcher> {
    fn from(queries: &HttpQueryParams) -> Self {
        queries.get_queries().iter()
            .filter(|q| q.is_case_insensitive())
            .map(QueryCaseInsensitiveMatcher::try_from).flatten()
            .collect_vec()
    }
}

impl TryFrom<&Query> for QueryCaseInsensitiveMatcher {
    type Error = anyhow::Error;

    fn try_from(query_matcher: &Query) -> anyhow::Result<Self> {
        query_matcher.equal_to_as_str()
            .filter(|_| query_matcher.is_case_insensitive())
            .map(|it| query_case_insensitive(query_matcher.key.to_string(), it))
            .ok_or_else(|| anyhow::Error::msg("No case insensitive query matcher found"))
    }
}
