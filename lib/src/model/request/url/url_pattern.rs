use std::str::FromStr;

use crate::wiremock::{Match, Request};
use crate::{StubrError, StubrResult};
use regex::Regex;

use super::HttpUrlStub;

pub struct UrlPatternMatcher(Regex);

impl Match for UrlPatternMatcher {
    fn matches(&self, req: &Request) -> bool {
        self.0.is_match(req.url.as_str())
    }
}

impl TryFrom<&HttpUrlStub> for UrlPatternMatcher {
    type Error = StubrError;

    fn try_from(http_url: &HttpUrlStub) -> StubrResult<Self> {
        http_url
            .url_pattern
            .as_ref()
            .and_then(|it| Regex::from_str(it).ok())
            .map(Self)
            .ok_or_else(|| StubrError::QuietError)
    }
}
