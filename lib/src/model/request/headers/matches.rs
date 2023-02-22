use std::{ops::Not, str::FromStr};

use crate::error::StubrResult;
use crate::wiremock::{Match, Request};
use crate::StubrError;
use http_types::headers::HeaderName;
use itertools::Itertools;
use regex::Regex;

use super::{super::matcher::RequestMatcherStub, HttpReqHeadersStub};

pub struct HeaderRegexMatcher(String, Regex, bool);

impl HeaderRegexMatcher {
    fn matches(&self, value: &str) -> bool {
        ((self.2) ^ (self.1.is_match(value))).not()
    }
}

impl Match for HeaderRegexMatcher {
    fn matches(&self, req: &Request) -> bool {
        HeaderName::from_str(self.0.as_str())
            .ok()
            .and_then(|key| req.headers.get(&key))
            .map(|values| self.matches(values.as_str()))
            .unwrap_or_default()
    }
}

impl TryFrom<&HttpReqHeadersStub> for Vec<HeaderRegexMatcher> {
    type Error = StubrError;

    fn try_from(headers: &HttpReqHeadersStub) -> StubrResult<Self> {
        headers.get_headers().ok_or_else(|| StubrError::QuietError).map(|iter| {
            iter.filter(|h| h.is_by_regex())
                .filter_map(|it| HeaderRegexMatcher::try_from(&it).ok())
                .collect_vec()
        })
    }
}

impl TryFrom<&RequestMatcherStub> for HeaderRegexMatcher {
    type Error = StubrError;

    fn try_from(header: &RequestMatcherStub) -> StubrResult<Self> {
        let maybe_positive_regex = header
            .matches_as_regex()
            .filter(|_| header.is_matches())
            .map(|it| HeaderRegexMatcher(header.key.to_string(), it, true));
        let maybe_negative_regex = || {
            header
                .does_not_match_as_regex()
                .filter(|_| header.is_does_not_matches())
                .map(|it| HeaderRegexMatcher(header.key.to_string(), it, false))
        };
        maybe_positive_regex
            .or_else(maybe_negative_regex)
            .ok_or_else(|| StubrError::QuietError)
    }
}
