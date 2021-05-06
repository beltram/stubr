use std::{convert::TryFrom, str::FromStr};

use http_types::headers::HeaderName;
use itertools::Itertools;
use wiremock::{Match, Request};

use super::{HttpReqHeadersStub, super::matcher::RequestMatcherStub};

pub struct HeaderContainsMatcher(String, String);

impl Match for HeaderContainsMatcher {
    fn matches(&self, req: &Request) -> bool {
        HeaderName::from_str(self.0.as_str()).ok()
            .and_then(|key| req.headers.get(&key))
            .map(|values| values.iter().any(|it| it.to_string().contains(self.1.as_str())))
            .unwrap_or_default()
    }
}

impl From<&HttpReqHeadersStub> for Vec<HeaderContainsMatcher> {
    fn from(headers: &HttpReqHeadersStub) -> Self {
        headers.get_headers().iter()
            .filter(|h| h.is_contains())
            .map(HeaderContainsMatcher::try_from).flatten()
            .collect_vec()
    }
}

impl TryFrom<&RequestMatcherStub> for HeaderContainsMatcher {
    type Error = anyhow::Error;

    fn try_from(header: &RequestMatcherStub) -> anyhow::Result<Self> {
        header.value.as_ref()
            .filter(|_| header.is_contains())
            .and_then(|it| it.contains.as_ref())
            .map(|contains| HeaderContainsMatcher(header.key.to_string(), contains.to_string()))
            .ok_or_else(|| anyhow::Error::msg("No header contains matcher found"))
    }
}
