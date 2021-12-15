use std::convert::TryFrom;

use jsonpath_lib::Compiled;
use serde_json::{from_slice as deserialize, Value};
use wiremock::{Match, Request};

use super::BodyPatternStub;

pub struct JsonPathMatcher(Compiled);

impl Match for JsonPathMatcher {
    fn matches(&self, req: &Request) -> bool {
        deserialize::<Value>(&req.body).ok().as_ref()
            .and_then(|it| self.0.select(it).ok())
            .filter(|it| !it.is_empty())
            .is_some()
    }
}

impl TryFrom<&BodyPatternStub> for JsonPathMatcher {
    type Error = anyhow::Error;

    fn try_from(body: &BodyPatternStub) -> anyhow::Result<Self> {
        body.matches_json_path.as_ref()
            .filter(|_| body.is_by_json_path())
            .and_then(|it| jsonpath_lib::Compiled::compile(it.as_str()).ok())
            .map(Self)
            .ok_or_else(|| anyhow::Error::msg("No json path matcher found"))
    }
}