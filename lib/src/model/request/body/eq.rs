use serde_json::Value;
use wiremock::{Match, Request};

use super::{BodyPatternStub, super::json::{eq::JsonExactMatcher, JsonMatcher}};

pub struct BodyExactMatcher(Value);

impl Match for BodyExactMatcher {
    fn matches(&self, request: &Request) -> bool {
        serde_json::from_slice(request.body.as_slice()).ok()
            .map(|body| JsonExactMatcher(&self.0).matches(&body))
            .unwrap_or_default()
    }
}

impl TryFrom<&BodyPatternStub> for BodyExactMatcher {
    type Error = anyhow::Error;

    fn try_from(body: &BodyPatternStub) -> anyhow::Result<Self> {
        let is_exact_matching = body.is_by_json_equality()
            && !body.is_ignore_extra_elements()
            && !body.is_ignore_array_order();
        body.equal_to_json.as_ref()
            .filter(|_| is_exact_matching)
            .map(|v| Self(v.to_owned()))
            .ok_or_else(|| anyhow::Error::msg(""))
    }
}