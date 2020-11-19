use std::convert::TryFrom;

use itertools::Itertools;
use regex::Regex;
use wiremock::{Match, Request};

use super::HttpQueryParamsDto;
use super::super::matcher::RequestMatcherDto;
use std::ops::Not;

pub struct QueryRegexMatcher(String, Regex, bool);

impl QueryRegexMatcher {
    fn matches(&self, value: &str) -> bool {
        ((self.2) ^ (self.1.is_match(value))).not()
    }
}

impl Match for QueryRegexMatcher {
    fn matches(&self, request: &Request) -> bool {
        request.url.query_pairs()
            .find(|(k, _)| k == self.0.as_str())
            .map(|(_, v)| self.matches(v.as_ref()))
            .unwrap_or_default()
    }
}

impl From<&HttpQueryParamsDto> for Vec<QueryRegexMatcher> {
    fn from(queries: &HttpQueryParamsDto) -> Self {
        queries.get_queries().iter()
            .filter(|q| q.is_by_regex())
            .map(QueryRegexMatcher::try_from).flatten()
            .collect_vec()
    }
}

impl TryFrom<&RequestMatcherDto> for QueryRegexMatcher {
    type Error = anyhow::Error;

    fn try_from(query: &RequestMatcherDto) -> anyhow::Result<Self> {
        let maybe_positive_regex = query.matches_as_regex()
            .filter(|_| query.is_matches())
            .map(|it| QueryRegexMatcher(query.key.to_string(), it, true));
        let maybe_negative_regex = || {
            query.does_not_match_as_regex()
                .filter(|_| query.is_does_not_matches())
                .map(|it| QueryRegexMatcher(query.key.to_string(), it, false))
        };
        maybe_positive_regex
            .or_else(maybe_negative_regex)
            .ok_or_else(|| anyhow::Error::msg("No query matcher by regex found"))
    }
}
