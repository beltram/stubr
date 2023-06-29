use crate::wiremock_rs::{Match, Request};
use crate::{StubrError, StubrResult};
use itertools::Itertools;

use super::{super::matcher::RequestMatcherStub, HttpQueryParamsStub};

pub struct QueryContainsMatcher(String, String);

impl Match for QueryContainsMatcher {
    fn matches(&self, req: &Request) -> bool {
        req.url
            .query_pairs()
            .find(|(k, _)| k == self.0.as_str())
            .map(|(_, v)| v.contains(self.1.as_str()))
            .unwrap_or_default()
    }
}

impl TryFrom<&HttpQueryParamsStub> for Vec<QueryContainsMatcher> {
    type Error = StubrError;

    fn try_from(queries: &HttpQueryParamsStub) -> StubrResult<Self> {
        queries.get_queries().ok_or_else(|| StubrError::QuietError).map(|iter| {
            iter.filter(|h| h.is_contains())
                .filter_map(|it| QueryContainsMatcher::try_from(&it).ok())
                .collect_vec()
        })
    }
}

impl TryFrom<&RequestMatcherStub> for QueryContainsMatcher {
    type Error = StubrError;

    fn try_from(query: &RequestMatcherStub) -> StubrResult<Self> {
        query
            .value
            .as_ref()
            .filter(|_| query.is_contains())
            .and_then(|it| it.contains.as_ref())
            .map(|it| QueryContainsMatcher(query.key.to_string(), it.to_string()))
            .ok_or_else(|| StubrError::QuietError)
    }
}
