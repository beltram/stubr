use serde_json::{from_slice as deserialize, Value};
use wiremock::{Match, Request};

use super::{
    BodyPatternStub,
    super::json::{json_path_contains::JsonPathContainsMatcher, JsonMatcher},
};

pub struct JsonBodyPathContainsMatcher(String, String);

impl Match for JsonBodyPathContainsMatcher {
    fn matches(&self, req: &Request) -> bool {
        deserialize::<Value>(&req.body).ok().as_ref()
            .map(|json| JsonPathContainsMatcher(&self.0, &self.1).matches(json))
            .unwrap_or_default()
    }
}

impl TryFrom<&BodyPatternStub> for JsonBodyPathContainsMatcher {
    type Error = anyhow::Error;

    fn try_from(body: &BodyPatternStub) -> anyhow::Result<Self> {
        body.expression.as_ref()
            .filter(|_| body.is_by_json_path_contains())
            .and_then(|path| body.contains.as_ref().map(|contains| (path, contains)))
            .map(|(path, contains)| Self(path.to_string(), contains.to_owned()))
            .ok_or_else(|| anyhow::Error::msg(""))
    }
}