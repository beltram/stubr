use std::str::FromStr;

use http_types::headers::HeaderName;
use serde::{Deserialize, Serialize};
use wiremock::MockBuilder;

use basic::{BasicAuthMatcher, BasicAuthStub};
use jwt::JwtAuthStub;

use super::MockRegistrable;

mod basic;
mod jwt;
mod helpers;

const BEARER_PREFIX: &'static str = "Bearer";

#[derive(Serialize, Deserialize, Debug, Default, Hash)]
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


lazy_static! {
    pub(crate) static ref AUTHORIZATION_HEADER: HeaderName = HeaderName::from_str("authorization").unwrap();
}