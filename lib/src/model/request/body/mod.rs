use std::convert::TryFrom;

use serde::Deserialize;
use serde_json::Value;
use wiremock::{matchers::BodyExactMatcher, MockBuilder};

use binary_eq::BinaryEqualMatcher;
use eq_relaxed::JsonBodyRelaxedMatcher;
use json_path::JsonPathMatcher;
use json_path_contains::JsonPathContainsMatcher;
use json_path_eq::JsonPathEqMatcher;

use super::MockRegistrable;

mod eq;
mod diff;
mod eq_relaxed;
mod json_path;
mod json_path_eq;
mod json_path_contains;
mod binary_eq;

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
    /// strict equality by bytes comparison
    binary_equal_to: Option<String>,
    /// used alongside [equalToJson].
    /// Instructs stubr not to fail when extra fields are present in request body.
    ignore_extra_elements: Option<bool>,
    /// used alongside [equalToJson].
    /// Any array present in request body will be matched by equality ignoring items order.
    ignore_array_order: Option<bool>,
}

impl BodyPatternDto {
    fn is_by_json_equality(&self) -> bool {
        self.equal_to_json.is_some()
            && self.matches_json_path.is_none()
            && self.expression.is_none()
            && self.binary_equal_to.is_none()
    }

    fn is_by_json_path(&self) -> bool {
        self.matches_json_path.is_some()
            && self.equal_to_json.is_none()
            && self.expression.is_none()
    }

    fn is_by_json_path_eq(&self) -> bool {
        self.expression.is_some()
            && self.equal_to_json.is_some()
    }

    fn is_by_json_path_contains(&self) -> bool {
        self.expression.is_some()
            && self.contains.is_some()
    }

    fn is_by_binary_equality(&self) -> bool {
        self.binary_equal_to.is_some()
    }

    fn is_ignore_extra_elements(&self) -> bool {
        self.ignore_extra_elements.unwrap_or_default()
    }

    fn is_ignore_array_order(&self) -> bool {
        self.ignore_array_order.unwrap_or_default()
    }
}

impl MockRegistrable for Vec<BodyPatternDto> {
    fn register(&self, mut mock: MockBuilder) -> MockBuilder {
        for body_pattern in self {
            if let Ok(exact_json) = BodyExactMatcher::try_from(body_pattern) {
                mock = mock.and(exact_json)
            }
            if let Ok(relaxed_exact_json) = JsonBodyRelaxedMatcher::try_from(body_pattern) {
                mock = mock.and(relaxed_exact_json)
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
            if let Ok(binary_equal) = BinaryEqualMatcher::try_from(body_pattern) {
                mock = mock.and(binary_equal)
            }
        }
        mock
    }
}