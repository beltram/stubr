use std::convert::TryFrom;

use serde::Deserialize;
use wiremock::{Match, Request};
use wiremock::matchers::{method, MethodExactMatcher};

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

pub struct MethodAnyMatcher;

impl Match for MethodAnyMatcher {
    fn matches(&self, _: &Request) -> bool {
        true
    }
}