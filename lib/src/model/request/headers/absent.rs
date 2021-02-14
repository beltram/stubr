use std::{convert::TryFrom, ops::Not, str::FromStr};

use http_types::headers::HeaderName;
use itertools::Itertools;
use wiremock::{Match, Request};

use super::{HttpReqHeadersDto, super::matcher::RequestMatcherDto};

pub struct HeaderAbsentMatcher(String, bool);

impl Match for HeaderAbsentMatcher {
    fn matches(&self, req: &Request) -> bool {
        HeaderName::from_str(self.0.as_str()).ok()
            .map(|key| ((req.headers.get(&key).is_none()) ^ (self.1)).not())
            .unwrap_or_default()
    }
}

impl From<&HttpReqHeadersDto> for Vec<HeaderAbsentMatcher> {
    fn from(headers: &HttpReqHeadersDto) -> Self {
        headers.get_headers().iter()
            .filter(|h| h.is_absent())
            .map(HeaderAbsentMatcher::try_from).flatten()
            .collect_vec()
    }
}

impl TryFrom<&RequestMatcherDto> for HeaderAbsentMatcher {
    type Error = anyhow::Error;

    fn try_from(header: &RequestMatcherDto) -> anyhow::Result<Self> {
        header.value.as_ref()
            .filter(|_| header.is_absent())
            .map(|it| HeaderAbsentMatcher(header.key.to_string(), it.absent.unwrap_or_default()))
            .ok_or_else(|| anyhow::Error::msg("No header absent matcher found"))
    }
}
