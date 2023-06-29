use crate::error::StubrResult;
use crate::wiremock_rs::{Match, Request};
use crate::StubrError;
use serde_json::Value;

use super::{
    super::json::{json_path_eq::JsonPathEqMatcher, JsonMatcher},
    BodyMatcherStub,
};

pub struct JsonBodyPathEqMatcher(String, Value);

impl JsonBodyPathEqMatcher {
    pub fn matching_json_path_eq(&self, bytes: &[u8]) -> bool {
        serde_json::from_slice::<Value>(bytes)
            .ok()
            .as_ref()
            .map(|json| JsonPathEqMatcher(&self.0, &self.1).matches(json))
            .unwrap_or_default()
    }
}

impl Match for JsonBodyPathEqMatcher {
    fn matches(&self, req: &Request) -> bool {
        self.matching_json_path_eq(&req.body)
    }
}

impl TryFrom<&BodyMatcherStub> for JsonBodyPathEqMatcher {
    type Error = StubrError;

    fn try_from(body: &BodyMatcherStub) -> StubrResult<Self> {
        body.expression
            .as_ref()
            .filter(|_| body.is_by_json_path_eq())
            .and_then(|path| body.equal_to_json.as_ref().map(|eq| (path, eq)))
            .map(|(path, eq)| Self(path.to_string(), eq.to_owned()))
            .ok_or_else(|| StubrError::QuietError)
    }
}
