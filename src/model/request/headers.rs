use std::convert::TryFrom;

use itertools::Itertools;
use serde::Deserialize;
use serde_json::{from_value, Map, Value};
use wiremock::matchers::{header, HeaderExactMatcher};
use wiremock::MockBuilder;

use super::super::request::MockRegistrable;
use super::header_insensitive_case::HeaderCaseInsensitiveMatcher;
use super::header_value::HeaderValue;

#[derive(Deserialize, Debug, Default)]
pub struct HttpReqHeaders {
    // matches all request http headers
    headers: Option<Map<String, Value>>,
}

impl MockRegistrable for HttpReqHeaders {
    fn register(&self, mut mock: MockBuilder) -> MockBuilder {
        for exact in self.exact_matchers() {
            mock = mock.and(exact);
        }
        for case_insensitive in self.case_insensitive_matchers() {
            mock = mock.and(case_insensitive);
        }
        mock
    }
}

impl HttpReqHeaders {
    fn exact_matchers(&self) -> Vec<HeaderExactMatcher> {
        if let Some(headers) = &self.headers {
            headers
                .iter()
                .map(|it| Header::try_from(it))
                .flatten()
                .filter(|h| !h.is_case_insensitive())
                .map(|it| HeaderExactMatcher::try_from(&it))
                .flatten()
                .collect_vec()
        } else {
            vec![]
        }
    }

    fn case_insensitive_matchers(&self) -> Vec<HeaderCaseInsensitiveMatcher> {
        if let Some(headers) = &self.headers {
            headers
                .iter()
                .map(|it| Header::try_from(it))
                .flatten()
                .filter(|h| h.is_case_insensitive())
                .map(|it| HeaderCaseInsensitiveMatcher::try_from(&it))
                .flatten()
                .collect_vec()
        } else {
            vec![]
        }
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct Header {
    // header key e.g. 'Content-Type'
    pub key: String,
    pub value: Option<HeaderValue>,
}

impl Header {
    fn is_case_insensitive(&self) -> bool {
        self.value
            .as_ref()
            .and_then(|v| v.case_insensitive)
            .map_or(false, |case| case)
    }
}

impl TryFrom<(&String, &Value)> for Header {
    type Error = anyhow::Error;

    fn try_from((k, v): (&String, &Value)) -> anyhow::Result<Self> {
        Ok(Self {
            key: k.to_owned(),
            value: from_value(v.to_owned()).ok(),
        })
    }
}

impl TryFrom<&Header> for HeaderExactMatcher {
    type Error = anyhow::Error;

    fn try_from(header_matcher: &Header) -> anyhow::Result<Self> {
        header_matcher
            .value
            .as_ref()
            .and_then(|it| it.equal_to.as_ref())
            .map(|exact| header(header_matcher.key.as_str(), exact.as_str()))
            .ok_or_else(|| anyhow::Error::msg("No exact header matcher found"))
    }
}
