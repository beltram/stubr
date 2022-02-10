use jsonpath_lib::Compiled;
use wiremock::{Match, Request};

use super::super::super::{
    helpers::RequestAuthExtension,
    super::json::{json_path::JsonPathMatcher, JsonMatcher},
};

pub struct JsonPayloadPathMatcher(Compiled);

impl JsonPayloadPathMatcher {
    pub fn new(path: &str) -> Option<Self> {
        jsonpath_lib::Compiled::compile(path).ok().map(Self)
    }
}

impl Match for JsonPayloadPathMatcher {
    fn matches(&self, req: &Request) -> bool {
        req.jwt_payload()
            .map(|p| JsonPathMatcher(&self.0).matches(&p))
            .unwrap_or_default()
    }
}