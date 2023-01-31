use std::hash::{Hash, Hasher};

use crate::wiremock::MockBuilder;

use super::super::MockRegistrable;

mod eq;
mod one_of;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct JwtAlgStub {
    equal_to: Option<String>,
    one_of: Option<Vec<jsonwebtoken::Algorithm>>,
}

impl MockRegistrable for JwtAlgStub {
    fn register(&self, mut mock: MockBuilder) -> MockBuilder {
        if let Ok(eq) = eq::JwtAlgExactMatcher::try_from(self) {
            mock = mock.and(eq)
        }
        if let Ok(one_of) = one_of::JwtAlgOneOfMatcher::try_from(self) {
            mock = mock.and(one_of)
        }
        mock
    }
}

impl Hash for JwtAlgStub {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.equal_to.as_ref().hash(state)
    }
}
