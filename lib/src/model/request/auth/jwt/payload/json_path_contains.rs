use crate::wiremock::{Match, Request};

use super::super::super::{
    super::json::{json_path_contains::JsonPathContainsMatcher, JsonMatcher},
    helpers::RequestAuthExtension,
};

pub struct JsonPayloadPathContainsMatcher(pub String, pub String);

impl Match for JsonPayloadPathContainsMatcher {
    fn matches(&self, req: &Request) -> bool {
        req.jwt_payload()
            .map(|p| JsonPathContainsMatcher(&self.0, &self.1).matches(&p))
            .unwrap_or_default()
    }
}
