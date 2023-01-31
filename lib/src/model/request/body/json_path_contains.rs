use crate::wiremock::{Match, Request};
use serde_json::Value;

use super::{
    super::json::{json_path_contains::JsonPathContainsMatcher, JsonMatcher},
    BodyMatcherStub,
};

pub struct JsonBodyPathContainsMatcher(String, String);

impl JsonBodyPathContainsMatcher {
    pub fn matching_json_path_contains(&self, bytes: &[u8]) -> bool {
        serde_json::from_slice::<Value>(bytes)
            .ok()
            .as_ref()
            .map(|json| JsonPathContainsMatcher(&self.0, &self.1).matches(json))
            .unwrap_or_default()
    }
}

impl Match for JsonBodyPathContainsMatcher {
    fn matches(&self, req: &Request) -> bool {
        self.matching_json_path_contains(&req.body)
    }
}

impl TryFrom<&BodyMatcherStub> for JsonBodyPathContainsMatcher {
    type Error = anyhow::Error;

    fn try_from(body: &BodyMatcherStub) -> anyhow::Result<Self> {
        body.expression
            .as_ref()
            .filter(|_| body.is_by_json_path_contains())
            .and_then(|path| body.contains.as_ref().map(|contains| (path, contains)))
            .map(|(path, contains)| Self(path.to_string(), contains.to_owned()))
            .ok_or_else(|| anyhow::Error::msg(""))
    }
}
