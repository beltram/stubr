use std::convert::TryFrom;

use serde::Deserialize;
use serde_json::Value;
use wiremock::{matchers::BodyExactMatcher, MockBuilder};

use json_path::JsonPathMatcher;
use json_path_contains::JsonPathContainsMatcher;
use json_path_eq::JsonPathEqMatcher;

use super::MockRegistrable;

mod eq;
mod json_path;
mod json_path_eq;
mod json_path_contains;

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct BodyPatternDto {
    /// strict equality
    equal_to_json: Option<Value>,
    /// json path matcher
    matches_json_path: Option<String>,
    /// json path matcher when combined with 'equal_to_json' or 'contains'
    expression: Option<String>,
    /// if matched json path also contains given string
    contains: Option<String>,
}

impl BodyPatternDto {
    fn is_by_json_equality(&self) -> bool {
        self.equal_to_json.is_some() && self.matches_json_path.is_none() && self.expression.is_none()
    }

    fn is_by_json_path(&self) -> bool {
        self.matches_json_path.is_some() && self.equal_to_json.is_none() && self.expression.is_none()
    }

    fn is_by_json_path_eq(&self) -> bool {
        self.expression.is_some() && self.equal_to_json.is_some()
    }

    fn is_by_json_path_contains(&self) -> bool {
        self.expression.is_some() && self.contains.is_some()
    }
}

impl MockRegistrable for Vec<BodyPatternDto> {
    fn register(&self, mut mock: MockBuilder) -> MockBuilder {
        for body_pattern in self {
            if let Ok(exact_json) = BodyExactMatcher::try_from(body_pattern) {
                mock = mock.and(exact_json)
            }
            if let Ok(json_path) = JsonPathMatcher::try_from(body_pattern) {
                mock = mock.and(json_path)
            }
            if let Ok(json_path_eq) = JsonPathEqMatcher::try_from(body_pattern) {
                mock = mock.and(json_path_eq)
            }
            if let Ok(json_path_contains) = JsonPathContainsMatcher::try_from(body_pattern) {
                mock = mock.and(json_path_contains)
            }
        }
        mock
    }
}