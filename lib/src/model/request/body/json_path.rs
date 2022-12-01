use jsonpath_lib::Compiled;
use serde_json::Value;
use wiremock::{Match, Request};

use super::{
    super::json::{json_path::JsonPathMatcher, JsonMatcher},
    BodyPatternStub,
};

pub struct JsonBodyPathMatcher(Compiled);

impl Match for JsonBodyPathMatcher {
    fn matches(&self, req: &Request) -> bool {
        serde_json::from_slice::<Value>(&req.body)
            .ok()
            .as_ref()
            .map(|json| JsonPathMatcher(&self.0).matches(json))
            .unwrap_or_default()
    }
}

impl TryFrom<&BodyPatternStub> for JsonBodyPathMatcher {
    type Error = anyhow::Error;

    fn try_from(body: &BodyPatternStub) -> anyhow::Result<Self> {
        body.matches_json_path
            .as_deref()
            .filter(|_| body.is_by_json_path())
            .and_then(|jsonpath| jsonpath_lib::Compiled::compile(jsonpath).ok())
            .map(Self)
            .ok_or_else(|| anyhow::Error::msg(""))
    }
}
