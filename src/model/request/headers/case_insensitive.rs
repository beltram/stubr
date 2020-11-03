use std::convert::TryFrom;
use std::str::FromStr;

use http_types::headers::HeaderName;
use itertools::Itertools;
use wiremock::{Match, Request};

use crate::model::request::headers::Header;

use super::HttpReqHeaders;

fn header_case_insensitive(key: String, value: String) -> HeaderCaseInsensitiveMatcher {
    HeaderCaseInsensitiveMatcher(key, value)
}

pub struct HeaderCaseInsensitiveMatcher(String, String);

impl Match for HeaderCaseInsensitiveMatcher {
    fn matches(&self, request: &Request) -> bool {
        HeaderName::from_str(self.0.as_str()).ok()
            .and_then(|key| request.headers.get(&key))
            .map_or(false, |header_values| {
                header_values.iter()
                    .any(|it| it.to_string().eq_ignore_ascii_case(&self.1))
            })
    }
}

impl TryFrom<&Header> for HeaderCaseInsensitiveMatcher {
    type Error = anyhow::Error;

    fn try_from(header_matcher: &Header) -> anyhow::Result<Self> {
        header_matcher.value.as_ref()
            .filter(|_| header_matcher.is_case_insensitive())
            .and_then(|it| it.equal_to.as_ref())
            .map(|exact| header_case_insensitive(header_matcher.key.to_string(), exact.to_string()))
            .ok_or_else(|| anyhow::Error::msg("No exact header matcher found"))
    }
}

impl From<&HttpReqHeaders> for Vec<HeaderCaseInsensitiveMatcher> {
    fn from(headers: &HttpReqHeaders) -> Self {
        headers.get_headers().iter()
            .filter(|h| h.is_case_insensitive())
            .map(|it| HeaderCaseInsensitiveMatcher::try_from(it)).flatten()
            .collect_vec()
    }
}