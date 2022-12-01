use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use wiremock::MockBuilder;

use super::MockRegistrable;

mod binary_eq;
mod diff;
pub mod eq;
mod eq_relaxed;
mod json_path;
mod json_path_contains;
mod json_path_eq;

#[derive(Serialize, Deserialize, Debug, Default, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BodyPatternStub {
    /// strict equality
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equal_to_json: Option<Value>,
    /// json path matcher
    #[serde(skip_serializing)]
    pub matches_json_path: Option<String>,
    /// json path matcher when combined with 'equal_to_json' or 'contains'
    #[serde(skip_serializing)]
    pub expression: Option<String>,
    /// if matched json path also contains given string
    #[serde(skip_serializing)]
    pub contains: Option<String>,
    /// strict equality by bytes comparison
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_equal_to: Option<String>,
    /// used alongside [equalToJson].
    /// Instructs stubr not to fail when extra fields are present in request body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_extra_elements: Option<bool>,
    /// used alongside [equalToJson].
    /// Any array present in request body will be matched by equality ignoring items order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_array_order: Option<bool>,
}

impl BodyPatternStub {
    fn is_by_json_equality(&self) -> bool {
        self.equal_to_json.is_some() && self.matches_json_path.is_none() && self.expression.is_none() && self.binary_equal_to.is_none()
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

impl MockRegistrable for Vec<BodyPatternStub> {
    fn register(&self, mut mock: MockBuilder) -> MockBuilder {
        for body_pattern in self {
            if let Ok(exact_json) = eq::BodyExactMatcher::try_from(body_pattern) {
                mock = mock.and(exact_json)
            }
            if let Ok(relaxed_exact_json) = eq_relaxed::JsonBodyRelaxedMatcher::try_from(body_pattern) {
                mock = mock.and(relaxed_exact_json)
            }
            if let Ok(json_path) = json_path::JsonBodyPathMatcher::try_from(body_pattern) {
                mock = mock.and(json_path)
            }
            if let Ok(json_path_eq) = json_path_eq::JsonBodyPathEqMatcher::try_from(body_pattern) {
                mock = mock.and(json_path_eq)
            }
            if let Ok(json_path_contains) = json_path_contains::JsonBodyPathContainsMatcher::try_from(body_pattern) {
                mock = mock.and(json_path_contains)
            }
            if let Ok(binary_equal) = binary_eq::BinaryEqualMatcher::try_from(body_pattern) {
                mock = mock.and(binary_equal)
            }
        }
        mock
    }
}

impl PartialEq for BodyPatternStub {
    fn eq(&self, other: &Self) -> bool {
        self.equal_to_json.as_ref().eq(&other.equal_to_json.as_ref())
            && self.matches_json_path.as_ref().eq(&other.matches_json_path.as_ref())
            && self.expression.as_ref().eq(&other.expression.as_ref())
            && self.contains.as_ref().eq(&other.contains.as_ref())
            && self.binary_equal_to.as_ref().eq(&other.binary_equal_to.as_ref())
            && self.ignore_extra_elements.as_ref().eq(&other.ignore_extra_elements.as_ref())
            && self.ignore_array_order.as_ref().eq(&other.ignore_array_order.as_ref())
    }
}

impl Hash for BodyPatternStub {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(it) = self.equal_to_json.as_ref() {
            it.to_string().hash(state)
        };
        self.matches_json_path.as_ref().hash(state);
        self.expression.as_ref().hash(state);
        self.contains.as_ref().hash(state);
        self.binary_equal_to.as_ref().hash(state);
        self.ignore_extra_elements.as_ref().hash(state);
        self.ignore_array_order.as_ref().hash(state);
    }
}
