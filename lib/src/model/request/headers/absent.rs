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

impl TryFrom<&HttpReqHeadersStub> for Vec<HeaderAbsentMatcher> {
    type Error = anyhow::Error;

    fn try_from(headers: &HttpReqHeadersStub) -> Result<Self, Self::Error> {
        headers.get_headers()
            .ok_or_else(|| anyhow::Error::msg(""))
            .map(|iter| {
                iter
                    .filter(|h| h.is_absent())
                    .filter_map(|it| HeaderAbsentMatcher::try_from(&it).ok())
                    .collect_vec()
            })
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
