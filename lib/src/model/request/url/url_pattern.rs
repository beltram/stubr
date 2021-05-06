use std::{convert::TryFrom, str::FromStr};

use regex::Regex;
use wiremock::{Match, Request};

use super::HttpUrlStub;

pub struct UrlPatternMatcher(Regex);

impl Match for UrlPatternMatcher {
    fn matches(&self, req: &Request) -> bool {
        self.0.is_match(req.url.as_str())
    }
}

impl TryFrom<&HttpUrlStub> for UrlPatternMatcher {
    type Error = anyhow::Error;

    fn try_from(http_url: &HttpUrlStub) -> anyhow::Result<Self> {
        http_url.url_pattern.as_ref()
            .and_then(|it| Regex::from_str(it).ok())
            .map(Self)
            .ok_or_else(|| anyhow::Error::msg("No 'urlPattern'"))
    }
}
