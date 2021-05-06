use std::convert::TryFrom;

use itertools::Itertools;
use wiremock::matchers::{header, HeaderExactMatcher};

use super::{HttpReqHeadersStub, super::matcher::RequestMatcherStub};

impl From<&HttpReqHeadersStub> for Vec<HeaderExactMatcher> {
    fn from(headers: &HttpReqHeadersStub) -> Self {
        headers.get_headers().iter()
            .filter(|h| h.is_exact_match())
            .map(HeaderExactMatcher::try_from).flatten()
            .collect_vec()
    }
}

impl TryFrom<&RequestMatcherStub> for HeaderExactMatcher {
    type Error = anyhow::Error;

    fn try_from(header_matcher: &RequestMatcherStub) -> anyhow::Result<Self> {
        header_matcher.equal_to_as_str()
            .map(|exact| header(header_matcher.key.as_str(), exact.as_str()))
            .ok_or_else(|| anyhow::Error::msg("No exact header matcher found"))
    }
}