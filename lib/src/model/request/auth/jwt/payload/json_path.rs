use crate::{
    error::{StubrError, StubrResult},
    model::request::{
        auth::helpers::RequestAuthExtension,
        json::{json_path::JsonPathMatcher, JsonMatcher},
    },
    wiremock::{Match, Request},
};

pub struct JsonPayloadPathMatcher(jsonpath_lib::Compiled);

impl TryFrom<&str> for JsonPayloadPathMatcher {
    type Error = StubrError;

    fn try_from(path: &str) -> StubrResult<Self> {
        jsonpath_lib::Compiled::compile(path)
            .map_err(StubrError::JsonPathError)
            .map(Self)
    }
}

impl Match for JsonPayloadPathMatcher {
    fn matches(&self, req: &Request) -> bool {
        req.jwt_payload()
            .map(|p| JsonPathMatcher(&self.0).matches(&p))
            .unwrap_or_default()
    }
}
