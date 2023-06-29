use crate::wiremock_rs::MockBuilder;

use super::super::{BodyMatcherStub, MockRegistrable};

mod alg;
mod eq;
mod payload;

#[derive(Debug, Clone, Default, Hash, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct JwtAuthStub {
    equal_to: Option<String>,
    payload_patterns: Option<Vec<BodyMatcherStub>>,
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
        if let Some(payload_patterns) = self.payload_patterns.as_ref() {
            mock = payload::JwtPayloadStub(payload_patterns.to_owned()).register(mock)
        }
        mock
    }
}
