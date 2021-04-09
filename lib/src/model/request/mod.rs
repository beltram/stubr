use std::convert::TryFrom;

use serde::Deserialize;
use wiremock::MockBuilder;

use body::BodyPatternDto;
use headers::HttpReqHeadersDto;
use method::HttpMethodDto;
use query::HttpQueryParamsDto;
use url::HttpUrlDto;
use crate::model::request::auth::AuthDto;

mod headers;
mod query;
mod url;
mod matcher;
mod method;
mod body;
mod auth;

#[derive(Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct RequestDto {
    method: HttpMethodDto,
    #[serde(flatten)]
    url: HttpUrlDto,
    #[serde(flatten)]
    headers: HttpReqHeadersDto,
    #[serde(flatten)]
    queries: HttpQueryParamsDto,
    body_patterns: Vec<BodyPatternDto>,
    #[serde(flatten)]
    auth: AuthDto,
}

impl TryFrom<&RequestDto> for MockBuilder {
    type Error = anyhow::Error;

    fn try_from(request: &RequestDto) -> anyhow::Result<Self> {
        let mut mock = MockBuilder::from(&request.method);
        mock = request.url.register(mock);
        mock = request.headers.register(mock);
        mock = request.queries.register(mock);
        mock = request.body_patterns.register(mock);
        mock = request.auth.register(mock);
        Ok(mock)
    }
}

/// Normalizes appending a struct into a Mock
trait MockRegistrable {
    fn register(&self, mock: MockBuilder) -> MockBuilder;
}