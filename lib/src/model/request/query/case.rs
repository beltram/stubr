use crate::wiremock_rs::{Match, Request};
use crate::{StubrError, StubrResult};
use itertools::Itertools;

use super::{super::matcher::RequestMatcherStub, HttpQueryParamsStub};

pub struct QueryCaseInsensitiveMatcher(String, String);

impl Match for QueryCaseInsensitiveMatcher {
    fn matches(&self, req: &Request) -> bool {
        req.url
            .query_pairs()
            .find(|(k, _)| k == self.0.as_str())
            .map(|(_, v)| v.eq_ignore_ascii_case(self.1.as_str()))
            .unwrap_or_default()
    }
}

impl TryFrom<&HttpQueryParamsStub> for Vec<QueryCaseInsensitiveMatcher> {
    type Error = StubrError;

    fn try_from(queries: &HttpQueryParamsStub) -> StubrResult<Self> {
        queries.get_queries().ok_or_else(|| StubrError::QuietError).map(|iter| {
            iter.filter(|q| q.is_case_insensitive())
                .filter_map(|it| QueryCaseInsensitiveMatcher::try_from(&it).ok())
                .collect_vec()
        })
    }
}

impl TryFrom<&RequestMatcherStub> for QueryCaseInsensitiveMatcher {
    type Error = StubrError;

    fn try_from(query: &RequestMatcherStub) -> StubrResult<Self> {
        query
            .equal_to_as_str()
            .filter(|_| query.is_case_insensitive())
            .map(|it| QueryCaseInsensitiveMatcher(query.key.to_string(), it))
            .ok_or_else(|| StubrError::QuietError)
    }
}
