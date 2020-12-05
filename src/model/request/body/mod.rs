use std::convert::TryFrom;

use serde::Deserialize;
use serde_json::Value;
use wiremock::{matchers::BodyExactMatcher, MockBuilder};

use json_path::JsonPathMatcher;

use super::MockRegistrable;

mod exact;
mod json_path;

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct BodyPatternDto {
    /// strict equality
    equal_to_json: Option<Value>,
    /// json path matcher
    matches_json_path: Option<String>,
}

impl MockRegistrable for Vec<BodyPatternDto> {
    fn register(&self, mut mock: MockBuilder) -> MockBuilder {
        for body_pattern in self {
            if let Ok(exact_json) = BodyExactMatcher::try_from(body_pattern) {
                mock = mock.and(exact_json)
            } else if let Ok(json_path_matcher) = JsonPathMatcher::try_from(body_pattern) {
                mock = mock.and(json_path_matcher)
            }
        }
        mock
    }
}