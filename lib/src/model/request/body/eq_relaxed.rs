use std::convert::TryFrom;

use serde_json::{from_slice as deserialize, Map, Value};
use wiremock::{Match, Request};

use super::BodyPatternDto;

pub struct JsonBodyRelaxedMatcher(Map<String, Value>);

impl Match for JsonBodyRelaxedMatcher {
    fn matches(&self, req: &Request) -> bool {
        let maybe_body = deserialize::<Value>(&req.body).ok();
        let maybe_body = maybe_body.as_ref().and_then(|it| it.as_object());
        if let Some(req_body) = maybe_body {
            self.0.iter().all(|(k, v)| {
                req_body.get(k).map(|it| it == v).unwrap_or_default()
            })
        } else { false }
    }
}

impl TryFrom<&BodyPatternDto> for JsonBodyRelaxedMatcher {
    type Error = anyhow::Error;

    fn try_from(body: &BodyPatternDto) -> anyhow::Result<Self> {
        body.equal_to_json.as_ref()
            .filter(|_| body.is_ignore_extra_elements() && body.is_by_json_equality())
            .and_then(|v| v.to_owned().as_object().map(|it| it.to_owned()))
            .map(Self)
            .ok_or_else(|| anyhow::Error::msg("No body matcher by relaxed json equality"))
    }
}