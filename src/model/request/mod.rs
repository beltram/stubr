use std::convert::TryFrom;

use serde::Deserialize;
use wiremock::{Mock, MockBuilder};
use wiremock::matchers::MethodExactMatcher;

use headers::HttpReqHeaders;
use method::HttpMethod;
use query::HttpQueryParams;
use url::HttpUrl;

mod headers;
mod query;
mod url;
mod matcher;
mod method;

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct Request {
    method: HttpMethod,
    #[serde(flatten)]
    url: HttpUrl,
    #[serde(flatten)]
    headers: HttpReqHeaders,
    #[serde(flatten)]
    queries: HttpQueryParams,
}

impl TryFrom<Request> for MockBuilder {
    type Error = anyhow::Error;

    fn try_from(request: Request) -> Result<Self, Self::Error> {
        let method = MethodExactMatcher::from(request.method);
        let mut mock = Mock::given(method);
        mock = request.url.register(mock);
        mock = request.headers.register(mock);
        mock = request.queries.register(mock);
        Ok(mock)
    }
}

/// Normalizes appending a struct into a Mock
trait MockRegistrable {
    fn register(&self, mock: MockBuilder) -> MockBuilder;
}
