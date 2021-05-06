use std::convert::TryFrom;

use itertools::Itertools;
use wiremock::{Match, Request};

use super::{HttpQueryParamsStub, super::matcher::RequestMatcherStub};

pub struct QueryContainsMatcher(String, String);

impl Match for QueryContainsMatcher {
    fn matches(&self, req: &Request) -> bool {
        req.url.query_pairs()
            .find(|(k, _)| k == self.0.as_str())
            .map(|(_, v)| v.contains(self.1.as_str()))
            .unwrap_or_default()
    }
}

impl From<&HttpQueryParamsStub> for Vec<QueryContainsMatcher> {
    fn from(queries: &HttpQueryParamsStub) -> Self {
        queries.get_queries().iter()
            .filter(|h| h.is_contains())
            .map(QueryContainsMatcher::try_from).flatten()
            .collect_vec()
    }
}

impl TryFrom<&RequestMatcherStub> for QueryContainsMatcher {
    type Error = anyhow::Error;

    fn try_from(query: &RequestMatcherStub) -> anyhow::Result<Self> {
        query.value.as_ref()
            .filter(|_| query.is_contains())
            .and_then(|it| it.contains.as_ref())
            .map(|it| QueryContainsMatcher(query.key.to_string(), it.to_string()))
            .ok_or_else(|| anyhow::Error::msg("No query contains matcher found"))
    }
}
