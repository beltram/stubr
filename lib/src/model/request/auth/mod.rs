use serde::{Deserialize, Serialize};
use wiremock::MockBuilder;

use basic::{BasicAuthMatcher, BasicAuthStub};

use super::MockRegistrable;

mod basic;

#[derive(Serialize, Deserialize, Debug, Default, Hash)]
#[serde(default, rename_all = "camelCase")]
pub struct AuthStub {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth: Option<BasicAuthStub>,
}

impl MockRegistrable for AuthStub {
    fn register(&self, mut mock: MockBuilder) -> MockBuilder {
        if let Some(basic_auth) = self.basic_auth.as_ref() {
            mock = mock.and(BasicAuthMatcher::from(basic_auth))
        }
        mock
    }
}