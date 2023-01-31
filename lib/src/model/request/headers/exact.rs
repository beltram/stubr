use crate::wiremock::matchers::{header, HeaderExactMatcher};
use itertools::Itertools;

use super::{super::matcher::RequestMatcherStub, HttpReqHeadersStub};

impl TryFrom<&HttpReqHeadersStub> for Vec<HeaderExactMatcher> {
    type Error = anyhow::Error;

    fn try_from(headers: &HttpReqHeadersStub) -> anyhow::Result<Self> {
        headers.get_headers().ok_or_else(|| anyhow::Error::msg("")).map(|iter| {
            iter.filter(|h| h.is_exact_match())
                .filter_map(|it| HeaderExactMatcher::try_from(&it).ok())
                .collect_vec()
        })
    }
}

impl TryFrom<&RequestMatcherStub> for HeaderExactMatcher {
    type Error = anyhow::Error;

    fn try_from(header_matcher: &RequestMatcherStub) -> anyhow::Result<Self> {
        header_matcher
            .equal_to_as_str()
            .map(|exact| header(header_matcher.key.as_str(), exact.as_str()))
            .ok_or_else(|| anyhow::Error::msg("No exact header matcher found"))
    }
}
