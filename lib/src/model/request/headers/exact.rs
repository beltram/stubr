use std::convert::TryFrom;

use itertools::Itertools;
use wiremock::matchers::{header, HeaderExactMatcher};

use super::{HttpReqHeadersDto, super::matcher::RequestMatcherDto};

impl From<&HttpReqHeadersDto> for Vec<HeaderExactMatcher> {
    fn from(headers: &HttpReqHeadersDto) -> Self {
        headers.get_headers().iter()
            .filter(|h| h.is_exact_match())
            .map(HeaderExactMatcher::try_from).flatten()
            .collect_vec()
    }
}

impl TryFrom<&RequestMatcherDto> for HeaderExactMatcher {
    type Error = anyhow::Error;

    fn try_from(header_matcher: &RequestMatcherDto) -> anyhow::Result<Self> {
        header_matcher.equal_to_as_str()
            .map(|exact| header(header_matcher.key.as_str(), exact.as_str()))
            .ok_or_else(|| anyhow::Error::msg("No exact header matcher found"))
    }
}