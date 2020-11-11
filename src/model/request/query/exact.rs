use std::convert::TryFrom;

use itertools::Itertools;
use wiremock::matchers::{query_param, QueryParamExactMatcher};

use super::HttpQueryParams;
use super::super::matcher::RequestMatcherDto;

impl From<&HttpQueryParams> for Vec<QueryParamExactMatcher> {
    fn from(queries: &HttpQueryParams) -> Self {
        queries.get_queries().iter()
            .filter(|q| q.is_exact_match())
            .map(QueryParamExactMatcher::try_from).flatten()
            .collect_vec()
    }
}

impl TryFrom<&RequestMatcherDto> for QueryParamExactMatcher {
    type Error = anyhow::Error;

    fn try_from(query: &RequestMatcherDto) -> anyhow::Result<Self> {
        query.equal_to_as_str()
            .map(|equal| query_param(query.key.as_str(), equal.as_str()))
            .ok_or_else(|| anyhow::Error::msg("No exact query matcher found"))
    }
}