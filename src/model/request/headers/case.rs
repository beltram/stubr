use std::convert::TryFrom;
use std::str::FromStr;

use http_types::headers::HeaderName;
use itertools::Itertools;
use wiremock::{Match, Request};

use crate::model::request::headers::Header;

use super::HttpReqHeaders;

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

impl TryFrom<&Header> for HeaderCaseInsensitiveMatcher {
    type Error = anyhow::Error;

    fn try_from(header: &Header) -> anyhow::Result<Self> {
        header.value.as_ref()
            .filter(|_| header.is_case_insensitive())
            .and_then(|it| it.equal_to.as_ref())
            .map(|case| HeaderCaseInsensitiveMatcher(header.key.to_string(), case.to_string()))
            .ok_or_else(|| anyhow::Error::msg("No case insensitive header matcher found"))
    }
}
