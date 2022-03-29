use wiremock::MockBuilder;

use json_path::JsonPayloadPathMatcher;

use super::super::{MockRegistrable, super::BodyPatternStub};

mod eq;
mod json_path;
mod json_path_eq;
mod json_path_contains;

pub struct JwtPayloadStub(pub Vec<BodyPatternStub>);

impl MockRegistrable for JwtPayloadStub {
    fn register(&self, mut mock: MockBuilder) -> MockBuilder {
        for body_pattern in &self.0 {
            if let Some(expression) = body_pattern.expression.as_ref() {
                if let Some(eq) = body_pattern.equal_to_json.as_ref() {
                    mock = mock.and(json_path_eq::JsonPayloadPathEqMatcher(expression.to_owned(), eq.to_owned()))
                } else if let Some(contains) = body_pattern.contains.as_ref() {
                    mock = mock.and(json_path_contains::JsonPayloadPathContainsMatcher(expression.to_owned(), contains.to_owned()))
                }
            } else if let Some(eq) = body_pattern.equal_to_json.as_ref() {
                mock = mock.and(eq::JsonPayloadEqMatcher(eq.to_owned()))
            } else if let Some(matcher) = body_pattern.matches_json_path.as_deref().and_then(|it| JsonPayloadPathMatcher::try_from(it).ok()) {
                mock = mock.and(matcher)
            }
        }
        mock
    }
}
