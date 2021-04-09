use serde::Deserialize;
use wiremock::MockBuilder;

use basic::{BasicAuthDto, BasicAuthMatcher};

use super::MockRegistrable;

mod basic;

#[derive(Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct AuthDto {
    basic_auth: Option<BasicAuthDto>,
}

impl MockRegistrable for AuthDto {
    fn register(&self, mut mock: MockBuilder) -> MockBuilder {
        if let Some(basic_auth) = self.basic_auth.as_ref() {
            mock = mock.and(BasicAuthMatcher::from(basic_auth))
        }
        mock
    }
}