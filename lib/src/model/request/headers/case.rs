use std::{convert::TryFrom, str::FromStr};

use http_types::headers::HeaderName;
use itertools::Itertools;
use wiremock::{Match, Request};

use super::{HttpReqHeadersStub, super::matcher::RequestMatcherStub};

pub struct HeaderCaseInsensitiveMatcher(String, String);

impl Match for HeaderCaseInsensitiveMatcher {
    fn matches(&self, req: &Request) -> bool {
        HeaderName::from_str(self.0.as_str()).ok()
            .and_then(|key| req.headers.get(&key))
            .map(|values| values.iter().any(|it| it.to_string().eq_ignore_ascii_case(self.1.as_str())))
            .unwrap_or_default()
    }
}

impl From<&HttpReqHeadersStub> for Vec<HeaderCaseInsensitiveMatcher> {
    fn from(headers: &HttpReqHeadersStub) -> Self {
        headers.get_headers().iter()
            .filter(|h| h.is_case_insensitive())
            .map(HeaderCaseInsensitiveMatcher::try_from).flatten()
            .collect_vec()
    }
}

impl TryFrom<&RequestMatcherStub> for HeaderCaseInsensitiveMatcher {
    type Error = anyhow::Error;

    fn try_from(header: &RequestMatcherStub) -> anyhow::Result<Self> {
        header.equal_to_as_str()
            .filter(|_| header.is_case_insensitive())
            .map(|it| HeaderCaseInsensitiveMatcher(header.key.to_string(), it))
            .ok_or_else(|| anyhow::Error::msg("No case insensitive header matcher found"))
    }
}
