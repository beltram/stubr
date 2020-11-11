use std::convert::TryFrom;

use itertools::Itertools;
use wiremock::{Match, Request};

use super::{HttpQueryParams, Query};

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

    fn try_from(query: &Query) -> anyhow::Result<Self> {
        query.value.as_ref()
            .filter(|_| query.is_contains())
            .and_then(|it| it.contains.as_ref())
            .map(|it| QueryContainsMatcher(query.key.to_string(), it.to_string()))
            .ok_or_else(|| anyhow::Error::msg("No query contains matcher found"))
    }
}
