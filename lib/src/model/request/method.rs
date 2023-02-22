use crate::{
    wiremock::{
        matchers::{method, MethodExactMatcher},
        Match, Mock, MockBuilder, Request,
    },
    StubrError, StubrResult,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub struct HttpMethodStub(pub Verb);

#[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Verb {
    Any,
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
    Connect,
    Trace,
}

impl From<&str> for HttpMethodStub {
    fn from(v: &str) -> Self {
        Self(match v {
            "GET" => Verb::Get,
            "POST" => Verb::Post,
            "PUT" => Verb::Put,
            "DELETE" => Verb::Delete,
            "PATCH" => Verb::Patch,
            "HEAD" => Verb::Head,
            "OPTIONS" => Verb::Options,
            "CONNECT" => Verb::Connect,
            "TRACE" => Verb::Trace,
            _ => Verb::Any,
        })
    }
}

impl Default for HttpMethodStub {
    fn default() -> Self {
        Self(Verb::Any)
    }
}

impl TryFrom<&HttpMethodStub> for MethodExactMatcher {
    type Error = StubrError;

    fn try_from(http_method: &HttpMethodStub) -> StubrResult<Self> {
        let m = &http_method.0;
        if m != &Verb::Any {
            Ok(method(format!("{m:?}").as_str()))
        } else {
            Err(StubrError::QuietError)
        }
    }
}

impl From<&HttpMethodStub> for MockBuilder {
    fn from(method: &HttpMethodStub) -> Self {
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
