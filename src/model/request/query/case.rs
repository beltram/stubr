use std::convert::TryFrom;

use itertools::Itertools;
use wiremock::{Match, Request};

use super::HttpQueryParamsDto;
use super::super::matcher::RequestMatcherDto;

pub struct QueryCaseInsensitiveMatcher(String, String);

impl Match for QueryCaseInsensitiveMatcher {
    fn matches(&self, request: &Request) -> bool {
        request.url.query_pairs()
            .find(|(k, _)| k == self.0.as_str())
            .map(|(_, v)| v.eq_ignore_ascii_case(self.1.as_str()))
            .unwrap_or_default()
    }
}

impl From<&HttpQueryParamsDto> for Vec<QueryCaseInsensitiveMatcher> {
    fn from(queries: &HttpQueryParamsDto) -> Self {
        queries.get_queries().iter()
            .filter(|q| q.is_case_insensitive())
            .map(QueryCaseInsensitiveMatcher::try_from).flatten()
            .collect_vec()
    }
}

impl TryFrom<&RequestMatcherDto> for QueryCaseInsensitiveMatcher {
    type Error = anyhow::Error;

    fn try_from(query: &RequestMatcherDto) -> anyhow::Result<Self> {
        query.equal_to_as_str()
            .filter(|_| query.is_case_insensitive())
            .map(|it| QueryCaseInsensitiveMatcher(query.key.to_string(), it))
            .ok_or_else(|| anyhow::Error::msg("No case insensitive query matcher found"))
    }
}
