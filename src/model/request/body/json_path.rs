use std::convert::TryFrom;

use jsonpath_lib::select as matches_json_path;
use serde_json::{from_slice as deserialize, Value};
use wiremock::{Match, Request};

use super::BodyPatternDto;

pub struct JsonPathMatcher(String);

impl Match for JsonPathMatcher {
    fn matches(&self, req: &Request) -> bool {
        deserialize::<Value>(&req.body).ok().as_ref()
            .and_then(|it| matches_json_path(it, self.0.as_ref()).ok())
            .filter(|it| !it.is_empty())
            .is_some()
    }
}

impl TryFrom<&BodyPatternDto> for JsonPathMatcher {
    type Error = anyhow::Error;

    fn try_from(body: &BodyPatternDto) -> anyhow::Result<Self> {
        body.matches_json_path.as_ref()
            .filter(|_| body.is_by_json_path())
            .map(|it| JsonPathMatcher(it.to_string()))
            .ok_or_else(|| anyhow::Error::msg("No json path matcher found"))
    }
}