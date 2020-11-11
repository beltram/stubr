use std::convert::TryFrom;
use std::str::FromStr;

use http_types::headers::HeaderName;
use itertools::Itertools;
use wiremock::{Match, Request};

use super::HttpReqHeaders;
use super::super::matcher::RequestMatcherDto;

pub struct HeaderCaseInsensitiveMatcher(String, String);

impl Match for HeaderCaseInsensitiveMatcher {
    fn matches(&self, request: &Request) -> bool {
        HeaderName::from_str(self.0.as_str()).ok()
            .and_then(|key| request.headers.get(&key))
            .map(|values| values.iter().any(|it| it.to_string().eq_ignore_ascii_case(self.1.as_str())))
            .unwrap_or_default()
    }
}

impl From<&HttpReqHeaders> for Vec<HeaderCaseInsensitiveMatcher> {
    fn from(headers: &HttpReqHeaders) -> Self {
        headers.get_headers().iter()
            .filter(|h| h.is_case_insensitive())
            .map(HeaderCaseInsensitiveMatcher::try_from).flatten()
            .collect_vec()
    }
}

impl TryFrom<&RequestMatcherDto> for HeaderCaseInsensitiveMatcher {
    type Error = anyhow::Error;

    fn try_from(header: &RequestMatcherDto) -> anyhow::Result<Self> {
        header.equal_to_as_str()
            .filter(|_| header.is_case_insensitive())
            .map(|it| HeaderCaseInsensitiveMatcher(header.key.to_string(), it))
            .ok_or_else(|| anyhow::Error::msg("No case insensitive header matcher found"))
    }
}
