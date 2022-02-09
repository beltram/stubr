use serde::{Deserialize, Serialize};
use wiremock::MockBuilder;

use eq::JwtExactMatcher;

use super::super::MockRegistrable;

mod eq;

#[derive(Serialize, Deserialize, Debug, Default, Hash)]
#[serde(default, rename_all = "camelCase")]
pub struct JwtAuthStub {
    equal_to: Option<String>,
}

impl MockRegistrable for JwtAuthStub {
    fn register(&self, mut mock: MockBuilder) -> MockBuilder {
        if let Ok(eq) = JwtExactMatcher::try_from(self) {
            mock = mock.and(eq);
        }
        mock
    }
}