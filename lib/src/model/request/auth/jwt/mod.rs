use serde::{Deserialize, Serialize};
use wiremock::MockBuilder;

use super::super::MockRegistrable;

mod eq;
mod alg;

#[derive(Serialize, Deserialize, Debug, Default, Hash)]
#[serde(default, rename_all = "camelCase")]
pub struct JwtAuthStub {
    equal_to: Option<String>,
    alg: Option<alg::JwtAlgStub>,
}

impl MockRegistrable for JwtAuthStub {
    fn register(&self, mut mock: MockBuilder) -> MockBuilder {
        if let Ok(eq) = eq::JwtExactMatcher::try_from(self) {
            mock = mock.and(eq)
        }
        if let Some(alg) = self.alg.as_ref() {
            mock = alg.register(mock)
        }
        mock
    }
}