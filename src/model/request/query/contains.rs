use std::convert::TryFrom;

use itertools::Itertools;
use wiremock::{Match, Request};

use super::{HttpQueryParams, Query};

fn query_contains(key: String, value: String) -> QueryContainsMatcher {
    QueryContainsMatcher(key, value)
}

pub struct QueryContainsMatcher(String, String);

impl Match for QueryContainsMatcher {
    fn matches(&self, request: &Request) -> bool {
        request.url.query_pairs()
            .find(|(k, _)| k == self.0.as_str())
            .map(|(_, v)| v.contains(self.1.as_str()))
            .unwrap_or_default()
    }
}

impl From<&HttpQueryParams> for Vec<QueryContainsMatcher> {
    fn from(queries: &HttpQueryParams) -> Self {
        queries.get_queries().iter()
            .filter(|h| h.is_contains())
            .map(QueryContainsMatcher::try_from).flatten()
            .collect_vec()
    }
}

impl TryFrom<&Query> for QueryContainsMatcher {
    type Error = anyhow::Error;

    fn try_from(query_matcher: &Query) -> anyhow::Result<Self> {
        query_matcher.value.as_ref()
            .filter(|_| query_matcher.is_contains())
            .and_then(|it| it.contains.as_ref())
            .map(|it| query_contains(query_matcher.key.to_string(), it.to_string()))
            .ok_or_else(|| anyhow::Error::msg("No case insensitive query matcher found"))
    }
}
