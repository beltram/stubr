use serde::{Deserialize, Serialize};
use wiremock::MockBuilder;

use eq::JwtAlgExactMatcher;

use super::super::MockRegistrable;

mod eq;

#[derive(Serialize, Deserialize, Debug, Default, Hash)]
#[serde(default, rename_all = "camelCase")]
pub struct JwtAlgStub {
    equal_to: Option<String>,
}

impl MockRegistrable for JwtAlgStub {
    fn register(&self, mut mock: MockBuilder) -> MockBuilder {
        if let Ok(eq) = JwtAlgExactMatcher::try_from(self) {
            mock = mock.and(eq)
        }
        mock
    }
}