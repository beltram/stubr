use crate::wiremock::matchers::{query_param, QueryParamExactMatcher};
use crate::{StubrError, StubrResult};
use itertools::Itertools;

use super::{super::matcher::RequestMatcherStub, HttpQueryParamsStub};

impl TryFrom<&HttpQueryParamsStub> for Vec<QueryParamExactMatcher> {
    type Error = StubrError;

    fn try_from(queries: &HttpQueryParamsStub) -> StubrResult<Self> {
        queries.get_queries().ok_or_else(|| StubrError::QuietError).map(|iter| {
            iter.filter(|q| q.is_exact_match())
                .filter_map(|it| QueryParamExactMatcher::try_from(&it).ok())
                .collect_vec()
        })
    }
}

impl TryFrom<&RequestMatcherStub> for QueryParamExactMatcher {
    type Error = StubrError;

    fn try_from(query: &RequestMatcherStub) -> StubrResult<Self> {
        query
            .equal_to_as_str()
            .map(|equal| query_param(query.key.as_str(), equal.as_str()))
            .ok_or_else(|| StubrError::QuietError)
    }
}
