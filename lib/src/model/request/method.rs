use std::convert::TryFrom;

use serde::Deserialize;
use wiremock::{
    Match,
    matchers::{method, MethodExactMatcher},
    Mock,
    MockBuilder,
    Request,
};

const METHOD_ANY: &str = "ANY";

#[derive(Deserialize, Debug, Default)]
pub struct HttpMethodDto(String);

impl TryFrom<HttpMethodDto> for MethodExactMatcher {
    type Error = anyhow::Error;

    fn try_from(http_method: HttpMethodDto) -> anyhow::Result<Self> {
        let m = http_method.0.as_str();
        if m != METHOD_ANY {
            Ok(method(m))
        } else {
            anyhow::Result::Err(anyhow::Error::msg(""))
        }
    }
}

impl From<HttpMethodDto> for MockBuilder {
    fn from(method: HttpMethodDto) -> Self {
        MethodExactMatcher::try_from(method)
            .map(Mock::given)
            .unwrap_or_else(|_| Mock::given(MethodAnyMatcher))
    }
}

struct MethodAnyMatcher;

impl Match for MethodAnyMatcher {
    fn matches(&self, _: &Request) -> bool {
        true
    }
}