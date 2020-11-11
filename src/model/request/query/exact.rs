use std::convert::TryFrom;

use itertools::Itertools;
use wiremock::matchers::{query_param, QueryParamExactMatcher};

use super::{HttpQueryParams, Query};

impl From<&HttpQueryParams> for Vec<QueryParamExactMatcher> {
    fn from(queries: &HttpQueryParams) -> Self {
        queries.get_queries().iter()
            .filter(|h| h.is_exact_match())
            .map(QueryParamExactMatcher::try_from).flatten()
            .collect_vec()
    }
}

impl TryFrom<&Query> for QueryParamExactMatcher {
    type Error = anyhow::Error;

    fn try_from(query_matcher: &Query) -> anyhow::Result<Self> {
        query_matcher.equal_to_as_str()
            .map(|exact| query_param(query_matcher.key.as_str(), exact.as_str()))
            .ok_or_else(|| anyhow::Error::msg("No exact header matcher found"))
    }
}