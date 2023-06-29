use std::ops::Not;

use crate::wiremock_rs::{Match, Request};
use crate::{StubrError, StubrResult};
use itertools::Itertools;
use regex::Regex;

use super::{super::matcher::RequestMatcherStub, HttpQueryParamsStub};

pub struct QueryRegexMatcher(String, Regex, bool);

impl QueryRegexMatcher {
    fn matches(&self, value: &str) -> bool {
        ((self.2) ^ (self.1.is_match(value))).not()
    }
}

impl Match for QueryRegexMatcher {
    fn matches(&self, req: &Request) -> bool {
        req.url
            .query_pairs()
            .find(|(k, _)| k == self.0.as_str())
            .map(|(_, v)| self.matches(v.as_ref()))
            .unwrap_or_default()
    }
}

impl TryFrom<&HttpQueryParamsStub> for Vec<QueryRegexMatcher> {
    type Error = StubrError;

    fn try_from(queries: &HttpQueryParamsStub) -> StubrResult<Self> {
        queries.get_queries().ok_or_else(|| StubrError::QuietError).map(|iter| {
            iter.filter(|q| q.is_by_regex())
                .filter_map(|it| QueryRegexMatcher::try_from(&it).ok())
                .collect_vec()
        })
    }
}

impl TryFrom<&RequestMatcherStub> for QueryRegexMatcher {
    type Error = StubrError;

    fn try_from(query: &RequestMatcherStub) -> StubrResult<Self> {
        let maybe_positive_regex = query
            .matches_as_regex()
            .filter(|_| query.is_matches())
            .map(|it| QueryRegexMatcher(query.key.to_string(), it, true));
        let maybe_negative_regex = || {
            query
                .does_not_match_as_regex()
                .filter(|_| query.is_does_not_matches())
                .map(|it| QueryRegexMatcher(query.key.to_string(), it, false))
        };
        maybe_positive_regex
            .or_else(maybe_negative_regex)
            .ok_or_else(|| StubrError::QuietError)
    }
}
