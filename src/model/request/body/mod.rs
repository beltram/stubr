use std::convert::TryFrom;

use serde::Deserialize;
use serde_json::Value;
use wiremock::matchers::BodyExactMatcher;
use wiremock::MockBuilder;

use super::MockRegistrable;

mod exact;

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct BodyPatternDto {
    /// strict equality
    equal_to_json: Option<Value>,
}

impl MockRegistrable for Vec<BodyPatternDto> {
    fn register(&self, mut mock: MockBuilder) -> MockBuilder {
        for body_pattern in self {
            if let Ok(exact_json) = BodyExactMatcher::try_from(body_pattern) {
                mock = mock.and(exact_json)
            }
        }
        mock
    }
}