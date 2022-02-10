use serde_json::Value;
use wiremock::{Match, Request};

use super::super::super::{
    helpers::RequestAuthExtension,
    super::json::{eq::JsonExactMatcher, JsonMatcher},
};

pub struct JsonPayloadEqMatcher(pub Value);

impl Match for JsonPayloadEqMatcher {
    fn matches(&self, req: &Request) -> bool {
        req.jwt_payload()
            .map(|p| JsonExactMatcher(&self.0).matches(&p))
            .unwrap_or_default()
    }
}