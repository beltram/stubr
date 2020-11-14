use std::convert::TryFrom;

use serde::Deserialize;
use wiremock::{Mock, MockBuilder};
use wiremock::matchers::MethodExactMatcher;

use headers::HttpReqHeadersDto;
use method::HttpMethodDto;
use query::HttpQueryParamsDto;
use url::HttpUrlDto;

mod headers;
mod query;
mod url;
mod matcher;
mod method;

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct Request {
    method: HttpMethodDto,
    #[serde(flatten)]
    url: HttpUrlDto,
    #[serde(flatten)]
    headers: HttpReqHeadersDto,
    #[serde(flatten)]
    queries: HttpQueryParamsDto,
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
