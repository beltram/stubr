use std::convert::TryFrom;

use serde::Deserialize;
use serde_json::{Map, Value};
use wiremock::matchers::{header, HeaderExactMatcher};
use wiremock::MockBuilder;

use crate::model::request::MockRegistrable;

#[derive(Deserialize, Debug, Default)]
pub struct HttpReqHeaders {
    // matches all request http headers
    headers: Option<Map<String, Value>>,
}

impl MockRegistrable for HttpReqHeaders {
    fn register(&self, mut mock: MockBuilder) -> MockBuilder {
        if let Some(headers) = &self.headers {
            let headers = headers
                .iter()
                .map(|it| Header::try_from(it))
                .map(|it| it.and_then(|m| HeaderExactMatcher::try_from(&m)).ok())
                .collect::<Vec<Option<HeaderExactMatcher>>>();
            for maybe_header_matcher in headers {
                if let Some(header_matcher) = maybe_header_matcher {
                    mock = mock.and(header_matcher);
                }
            }
        }
        mock
    }
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct Header {
    key: String,
    // matches a header value exactly
    equal_to: Option<String>,
    // should header exact matching be case insensitive
    case_insensitive: Option<bool>,
}

impl TryFrom<(&String, &Value)> for Header {
    type Error = anyhow::Error;

    fn try_from((k, v): (&String, &Value)) -> anyhow::Result<Self> {
        Ok(Self {
            key: k.to_owned(),
            equal_to: v
                .as_object()
                .and_then(|it| it.get("equalTo"))
                .and_then(|it| it.as_str())
                .map(|it| it.to_owned()),
            ..Default::default()
        })
    }
}

impl TryFrom<&Header> for HeaderExactMatcher {
    type Error = anyhow::Error;

    fn try_from(header_matcher: &Header) -> anyhow::Result<Self> {
        if let Some(exact) = &header_matcher.equal_to {
            Ok(header(header_matcher.key.as_str(), exact.as_str()))
        } else {
            Err(anyhow::Error::msg("Cannot make into matcher"))
        }
    }
}
