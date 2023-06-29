use std::str::FromStr;

use crate::error::StubrResult;
use crate::wiremock_rs::{Match, Request};
use crate::StubrError;
use http_types::headers::HeaderName;
use itertools::Itertools;

use super::{super::matcher::RequestMatcherStub, HttpReqHeadersStub};

pub struct HeaderAbsentMatcher(String, bool);

impl Match for HeaderAbsentMatcher {
    fn matches(&self, req: &Request) -> bool {
        HeaderName::from_str(self.0.as_str())
            .ok()
            .map(|key| req.headers.get(&key).is_none() == self.1)
            .unwrap_or_default()
    }
}

impl TryFrom<&HttpReqHeadersStub> for Vec<HeaderAbsentMatcher> {
    type Error = StubrError;

    fn try_from(headers: &HttpReqHeadersStub) -> StubrResult<Self> {
        headers.get_headers().ok_or_else(|| StubrError::QuietError).map(|iter| {
            iter.filter(|h| h.is_absent())
                .filter_map(|it| HeaderAbsentMatcher::try_from(&it).ok())
                .collect_vec()
        })
    }
}

impl TryFrom<&RequestMatcherStub> for HeaderAbsentMatcher {
    type Error = StubrError;

    fn try_from(header: &RequestMatcherStub) -> StubrResult<Self> {
        header
            .value
            .as_ref()
            .filter(|_| header.is_absent())
            .map(|it| HeaderAbsentMatcher(header.key.to_string(), it.absent.unwrap_or_default()))
            .ok_or_else(|| StubrError::QuietError)
    }
}
