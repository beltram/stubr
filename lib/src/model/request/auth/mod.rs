use std::str::FromStr;

use crate::wiremock_rs::MockBuilder;
use http_types::headers::HeaderName;

use basic::{BasicAuthMatcher, BasicAuthStub};
use jwt::JwtAuthStub;

use super::MockRegistrable;

mod basic;
mod helpers;
mod jwt;

const BEARER_PREFIX: &str = "Bearer";

lazy_static! {
    pub(crate) static ref AUTHORIZATION_HEADER: HeaderName = HeaderName::from_str("authorization").expect("Implementation error");
}

#[derive(Debug, Clone, Default, Hash, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct AuthStub {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth: Option<BasicAuthStub>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_auth: Option<JwtAuthStub>,
}

impl MockRegistrable for AuthStub {
    fn register(&self, mut mock: MockBuilder) -> MockBuilder {
        if let Some(basic_auth) = self.basic_auth.as_ref() {
            mock = mock.and(BasicAuthMatcher::from(basic_auth))
        }
        if let Some(jwt) = self.jwt_auth.as_ref() {
            mock = jwt.register(mock)
        }
        mock
    }
}
