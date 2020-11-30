use std::convert::TryFrom;
use std::ops::Not;
use std::str::FromStr;

use http_types::headers::HeaderName;
use itertools::Itertools;
use regex::Regex;
use wiremock::{Match, Request};

use super::HttpReqHeadersDto;
use super::super::matcher::RequestMatcherDto;

pub struct HeaderRegexMatcher(String, Regex, bool);

impl HeaderRegexMatcher {
    fn matches(&self, value: &str) -> bool {
        ((self.2) ^ (self.1.is_match(value))).not()
    }
}

impl Match for HeaderRegexMatcher {
    fn matches(&self, req: &Request) -> bool {
        HeaderName::from_str(self.0.as_str()).ok()
            .and_then(|key| req.headers.get(&key))
            .map(|values| self.matches(values.as_str()))
            .unwrap_or_default()
    }
}

impl From<&HttpReqHeadersDto> for Vec<HeaderRegexMatcher> {
    fn from(headers: &HttpReqHeadersDto) -> Self {
        headers.get_headers().iter()
            .filter(|h| h.is_by_regex())
            .map(HeaderRegexMatcher::try_from).flatten()
            .collect_vec()
    }
}

impl TryFrom<&RequestMatcherDto> for HeaderRegexMatcher {
    type Error = anyhow::Error;

    fn try_from(header: &RequestMatcherDto) -> anyhow::Result<Self> {
        let maybe_positive_regex = header.matches_as_regex()
            .filter(|_| header.is_matches())
            .map(|it| HeaderRegexMatcher(header.key.to_string(), it, true));
        let maybe_negative_regex = || {
            header.does_not_match_as_regex()
                .filter(|_| header.is_does_not_matches())
                .map(|it| HeaderRegexMatcher(header.key.to_string(), it, false))
        };
        maybe_positive_regex
            .or_else(maybe_negative_regex)
            .ok_or_else(|| anyhow::Error::msg("No header matcher by regex found"))
    }
}
