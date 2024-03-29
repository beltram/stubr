use std::str::FromStr;

use crate::error::StubrResult;
use crate::wiremock_rs::{Match, Request};
use crate::StubrError;
use http_types::headers::HeaderName;
use itertools::Itertools;

use super::{super::matcher::RequestMatcherStub, HttpReqHeadersStub};

pub struct HeaderContainsMatcher(String, String);

impl Match for HeaderContainsMatcher {
    fn matches(&self, req: &Request) -> bool {
        HeaderName::from_str(self.0.as_str())
            .ok()
            .and_then(|key| req.headers.get(&key))
            .map(|values| values.iter().any(|it| it.to_string().contains(self.1.as_str())))
            .unwrap_or_default()
    }
}

impl TryFrom<&HttpReqHeadersStub> for Vec<HeaderContainsMatcher> {
    type Error = StubrError;

    fn try_from(headers: &HttpReqHeadersStub) -> StubrResult<Self> {
        headers.get_headers().ok_or_else(|| StubrError::QuietError).map(|iter| {
            iter.filter(|h| h.is_contains())
                .filter_map(|it| HeaderContainsMatcher::try_from(&it).ok())
                .collect_vec()
        })
    }
}

impl TryFrom<&RequestMatcherStub> for HeaderContainsMatcher {
    type Error = StubrError;

    fn try_from(header: &RequestMatcherStub) -> StubrResult<Self> {
        header
            .value
            .as_ref()
            .filter(|_| header.is_contains())
            .and_then(|it| it.contains.as_ref())
            .map(|contains| HeaderContainsMatcher(header.key.to_string(), contains.to_string()))
            .ok_or_else(|| StubrError::QuietError)
    }
}
