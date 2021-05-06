use std::{convert::TryFrom, str::FromStr};

use http_types::headers::HeaderName;
use itertools::Itertools;
use wiremock::{Match, Request};

use super::{HttpReqHeadersStub, super::matcher::RequestMatcherStub};

pub struct HeaderAbsentMatcher(String, bool);

impl Match for HeaderAbsentMatcher {
    fn matches(&self, req: &Request) -> bool {
        HeaderName::from_str(self.0.as_str()).ok()
            .map(|key| req.headers.get(&key).is_none() == self.1)
            .unwrap_or_default()
    }
}

impl From<&HttpReqHeadersStub> for Vec<HeaderAbsentMatcher> {
    fn from(headers: &HttpReqHeadersStub) -> Self {
        headers.get_headers().iter()
            .filter(|h| h.is_absent())
            .map(HeaderAbsentMatcher::try_from)
            .flatten()
            .collect_vec()
    }
}

impl TryFrom<&RequestMatcherStub> for HeaderAbsentMatcher {
    type Error = anyhow::Error;

    fn try_from(header: &RequestMatcherStub) -> anyhow::Result<Self> {
        header.value.as_ref()
            .filter(|_| header.is_absent())
            .map(|it| HeaderAbsentMatcher(header.key.to_string(), it.absent.unwrap_or_default()))
            .ok_or_else(|| anyhow::Error::msg("No header absent matcher found"))
    }
}
