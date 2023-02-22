use crate::wiremock::matchers::{path, query_param, PathExactMatcher, QueryParamExactMatcher};
use crate::{StubrError, StubrResult};
use http_types::Url;
use itertools::Itertools;

use super::HttpUrlStub;

pub struct ExactPathAndQueryMatcher(pub PathExactMatcher, pub Vec<QueryParamExactMatcher>);

impl TryFrom<&HttpUrlStub> for ExactPathAndQueryMatcher {
    type Error = StubrError;

    fn try_from(http_url: &HttpUrlStub) -> StubrResult<Self> {
        Url::try_from(http_url).map(Self::from)
    }
}

impl TryFrom<&HttpUrlStub> for Url {
    type Error = StubrError;

    fn try_from(http_url: &HttpUrlStub) -> StubrResult<Self> {
        http_url
            .url
            .as_ref()
            .map(|it| format!("http://localhost{it}"))
            .map(|it| Url::parse(&it))
            .transpose()?
            .ok_or_else(|| StubrError::QuietError)
    }
}

impl From<Url> for ExactPathAndQueryMatcher {
    fn from(url: Url) -> Self {
        let query_matchers = url.query_pairs().map(|(k, v)| query_param(k, v)).collect_vec();
        Self(path(url.path()), query_matchers)
    }
}
