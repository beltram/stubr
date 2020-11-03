use std::convert::TryFrom;

use serde::Deserialize;
use wiremock::{Mock, MockBuilder};
use wiremock::matchers::{method, MethodExactMatcher};

use headers::HttpReqHeaders;
use url::HttpUrl;

pub mod headers;
pub mod url;

#[derive(Deserialize, Debug, Default)]
struct HttpMethod(String);

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct Request {
    method: HttpMethod,
    #[serde(flatten)]
    url: HttpUrl,
    #[serde(flatten)]
    header: HttpReqHeaders,
}

impl TryFrom<Request> for MockBuilder {
    type Error = anyhow::Error;

    fn try_from(request: Request) -> Result<Self, Self::Error> {
        let method: MethodExactMatcher = request.method.into();
        let mut mock = Mock::given(method);
        mock = request.url.register(mock);
        mock = request.header.register(mock);
        Ok(mock)
    }
}

impl From<HttpMethod> for MethodExactMatcher {
    fn from(http_method: HttpMethod) -> Self {
        method(http_method.0.as_str())
    }
}

trait MockRegistrable {
    fn register(&self, mock: MockBuilder) -> MockBuilder;
}
