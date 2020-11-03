use std::convert::TryFrom;

use itertools::Itertools;
use wiremock::matchers::{header, HeaderExactMatcher};

use super::{Header, HttpReqHeaders};

impl From<&HttpReqHeaders> for Vec<HeaderExactMatcher> {
    fn from(headers: &HttpReqHeaders) -> Self {
        headers.get_headers().iter()
            .filter(|h| h.is_exact_match())
            .map(HeaderExactMatcher::try_from).flatten()
            .collect_vec()
    }
}

impl TryFrom<&Header> for HeaderExactMatcher {
    type Error = anyhow::Error;

    fn try_from(header_matcher: &Header) -> anyhow::Result<Self> {
        header_matcher.value.as_ref()
            .and_then(|it| it.equal_to.as_ref())
            .map(|exact| header(header_matcher.key.as_str(), exact.as_str()))
            .ok_or_else(|| anyhow::Error::msg("No exact header matcher found"))
    }
}