use std::convert::TryFrom;

use jsonpath_lib::select as matches_json_path;
use serde_json::from_slice as deserialize;
use serde_json::Value;
use wiremock::{Match, Request};

use super::BodyPatternDto;

pub struct JsonPathEqMatcher(String, Value);

impl Match for JsonPathEqMatcher {
    fn matches(&self, req: &Request) -> bool {
        deserialize::<Value>(&req.body).ok().as_ref()
            .and_then(|it| matches_json_path(it, &self.0).ok())
            .filter(|matched| !matched.is_empty())
            .filter(|matched| matched.iter().all(|&it| it == &self.1))
            .is_some()
    }
}

impl TryFrom<&BodyPatternDto> for JsonPathEqMatcher {
    type Error = anyhow::Error;

    fn try_from(body: &BodyPatternDto) -> anyhow::Result<Self> {
        if body.is_by_json_path_eq() {
            body.matches_json_path.as_ref()
                .and_then(|path| body.equal_to_json.as_ref().map(|eq| (path, eq)))
                .map(|(path, eq)| JsonPathEqMatcher(path.to_string(), eq.to_owned()))
                .ok_or_else(|| anyhow::Error::msg("No json path with eq matcher found"))
        } else { anyhow::Result::Err(anyhow::Error::msg("No json path with eq matcher found")) }
    }
}