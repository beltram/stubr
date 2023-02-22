use crate::error::StubrResult;
use crate::wiremock::{Match, Request};
use crate::StubrError;
use serde_json::Value;

use super::{
    super::json::{eq::JsonExactMatcher, JsonMatcher},
    BodyMatcherStub,
};

pub struct BodyExactMatcher(pub Value);

impl Match for BodyExactMatcher {
    fn matches(&self, request: &Request) -> bool {
        self.matching_exact_json(request.body.as_slice())
    }
}

impl BodyExactMatcher {
    pub fn matching_exact_json(&self, bytes: &[u8]) -> bool {
        serde_json::from_slice(bytes)
            .ok()
            .map(|body| JsonExactMatcher(&self.0).matches(&body))
            .unwrap_or_default()
    }
}

impl TryFrom<&BodyMatcherStub> for BodyExactMatcher {
    type Error = StubrError;

    fn try_from(matcher: &BodyMatcherStub) -> StubrResult<Self> {
        matcher
            .equal_to_json
            .as_ref()
            .filter(|_| matcher.is_exact_matching())
            .map(|v| Self(v.to_owned()))
            .ok_or_else(|| StubrError::QuietError)
    }
}
