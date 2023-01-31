use crate::wiremock::{Match, Request};
use jsonpath_lib::Compiled;
use serde_json::Value;

use super::{
    super::json::{json_path::JsonPathMatcher, JsonMatcher},
    BodyMatcherStub,
};

pub struct JsonPathBodyMatcher(Compiled);

impl JsonPathBodyMatcher {
    pub fn matching_json_path(&self, bytes: &[u8]) -> bool {
        serde_json::from_slice::<Value>(bytes)
            .ok()
            .as_ref()
            .map(|json| JsonPathMatcher(&self.0).matches(json))
            .unwrap_or_default()
    }
}

impl Match for JsonPathBodyMatcher {
    fn matches(&self, req: &Request) -> bool {
        self.matching_json_path(&req.body)
    }
}

impl TryFrom<&BodyMatcherStub> for JsonPathBodyMatcher {
    type Error = anyhow::Error;

    fn try_from(body: &BodyMatcherStub) -> anyhow::Result<Self> {
        body.matches_json_path
            .as_deref()
            .filter(|_| body.is_by_json_path())
            .and_then(|jsonpath| jsonpath_lib::Compiled::compile(jsonpath).ok())
            .map(Self)
            .ok_or_else(|| anyhow::Error::msg(""))
    }
}
