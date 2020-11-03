use std::convert::TryFrom;
use std::str::FromStr;

use http_types::headers::HeaderName;
use itertools::Itertools;
use wiremock::{Match, Request};

use crate::model::request::headers::Header;

use super::HttpReqHeaders;

fn header_contains(key: String, value: String) -> HeaderContainsMatcher {
    HeaderContainsMatcher(key, value)
}

pub struct HeaderContainsMatcher(String, String);

impl Match for HeaderContainsMatcher {
    fn matches(&self, request: &Request) -> bool {
        HeaderName::from_str(self.0.as_str()).ok()
            .and_then(|key| request.headers.get(&key))
            .map(|values| {
                values.iter().any(|it| it.to_string().contains(&self.1))
            })
            .unwrap_or_default()
    }
}

impl From<&HttpReqHeaders> for Vec<HeaderContainsMatcher> {
    fn from(headers: &HttpReqHeaders) -> Self {
        headers.get_headers().iter()
            .filter(|h| h.is_contains())
            .map(HeaderContainsMatcher::try_from).flatten()
            .collect_vec()
    }
}

impl TryFrom<&Header> for HeaderContainsMatcher {
    type Error = anyhow::Error;

    fn try_from(header_matcher: &Header) -> anyhow::Result<Self> {
        header_matcher.value.as_ref()
            .filter(|_| header_matcher.is_contains())
            .and_then(|it| it.contains.as_ref())
            .map(|contains| header_contains(header_matcher.key.to_string(), contains.to_string()))
            .ok_or_else(|| anyhow::Error::msg("No contains header matcher found"))
    }
}
