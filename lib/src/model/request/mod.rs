use serde::{Deserialize, Serialize};
use wiremock::MockBuilder;

use body::BodyPatternStub;
use headers::HttpReqHeadersStub;
use method::HttpMethodStub;
use query::HttpQueryParamsStub;
use url::HttpUrlStub;

use crate::model::request::auth::AuthStub;

pub mod auth;
pub mod body;
pub mod headers;
pub mod json;
pub mod matcher;
pub mod method;
pub mod query;
pub mod url;

#[derive(Serialize, Deserialize, Debug, Default, Hash)]
#[serde(default, rename_all = "camelCase")]
pub struct RequestStub {
    #[serde(default)]
    pub method: HttpMethodStub,
    #[serde(flatten)]
    pub url: HttpUrlStub,
    #[serde(flatten)]
    pub headers: HttpReqHeadersStub,
    #[serde(flatten)]
    pub queries: HttpQueryParamsStub,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub body_patterns: Vec<BodyPatternStub>,
    #[serde(flatten, skip_serializing)]
    pub auth: AuthStub,
}

impl TryFrom<&RequestStub> for MockBuilder {
    type Error = anyhow::Error;

    fn try_from(request: &RequestStub) -> anyhow::Result<Self> {
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
