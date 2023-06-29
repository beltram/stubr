use crate::wiremock_rs::{Match, Request};
use serde_json::Value;

use super::super::super::{
    super::json::{eq::JsonExactMatcher, JsonMatcher},
    helpers::RequestAuthExtension,
};

pub struct JsonPayloadEqMatcher(pub Value);

impl Match for JsonPayloadEqMatcher {
    fn matches(&self, req: &Request) -> bool {
        req.jwt_payload()
            .map(|p| JsonExactMatcher(&self.0).matches(&p))
            .unwrap_or_default()
    }
}
