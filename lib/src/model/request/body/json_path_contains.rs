use jsonpath_lib::select as matches_json_path;
use serde_json::{from_slice as deserialize, Value};
use wiremock::{Match, Request};

use super::BodyPatternStub;

pub struct JsonPathContainsMatcher(String, String);

impl Match for JsonPathContainsMatcher {
    fn matches(&self, req: &Request) -> bool {
        deserialize::<Value>(&req.body).ok().as_ref()
            .and_then(|it| matches_json_path(it, &self.0).ok())
            .filter(|matched| !matched.is_empty())
            .filter(|matched| {
                matched.iter()
                    .all(|it| it.as_str().map(|c| c.contains(self.1.as_str())).unwrap_or_default())
            })
            .is_some()
    }
}

impl TryFrom<&BodyPatternStub> for JsonPathContainsMatcher {
    type Error = anyhow::Error;

    fn try_from(body: &BodyPatternStub) -> anyhow::Result<Self> {
        body.expression.as_ref()
            .filter(|_| body.is_by_json_path_contains())
            .and_then(|path| body.contains.as_ref().map(|contains| (path, contains)))
            .map(|(path, contains)| Self(path.to_string(), contains.to_owned()))
            .ok_or_else(|| anyhow::Error::msg(""))
    }
}