use crate::wiremock::{Match, Request};
use jsonpath_lib::Compiled;

use super::super::super::{
    super::json::{json_path::JsonPathMatcher, JsonMatcher},
    helpers::RequestAuthExtension,
};

pub struct JsonPayloadPathMatcher(Compiled);

impl TryFrom<&str> for JsonPayloadPathMatcher {
    type Error = anyhow::Error;

    fn try_from(path: &str) -> anyhow::Result<Self> {
        jsonpath_lib::Compiled::compile(path).map(Self).map_err(anyhow::Error::msg)
    }
}

impl Match for JsonPayloadPathMatcher {
    fn matches(&self, req: &Request) -> bool {
        req.jwt_payload()
            .map(|p| JsonPathMatcher(&self.0).matches(&p))
            .unwrap_or_default()
    }
}
