use serde_json::{from_slice as deserialize, Value};
use wiremock::{Match, Request};

use super::{
    super::json::{json_path_eq::JsonPathEqMatcher, JsonMatcher},
    BodyPatternStub,
};

pub struct JsonBodyPathEqMatcher(String, Value);

impl Match for JsonBodyPathEqMatcher {
    fn matches(&self, req: &Request) -> bool {
        deserialize::<Value>(&req.body)
            .ok()
            .as_ref()
            .map(|json| JsonPathEqMatcher(&self.0, &self.1).matches(json))
            .unwrap_or_default()
    }
}

impl TryFrom<&BodyPatternStub> for JsonBodyPathEqMatcher {
    type Error = anyhow::Error;

    fn try_from(body: &BodyPatternStub) -> anyhow::Result<Self> {
        body.expression
            .as_ref()
            .filter(|_| body.is_by_json_path_eq())
            .and_then(|path| body.equal_to_json.as_ref().map(|eq| (path, eq)))
            .map(|(path, eq)| Self(path.to_string(), eq.to_owned()))
            .ok_or_else(|| anyhow::Error::msg(""))
    }
}
