use serde_json::{from_slice as deserialize, Value};
use wiremock::{Match, Request};

use super::{
    diff::{all::RelaxedValue, array::RelaxedJsonArray, extra::RelaxedExtraJsonObj},
    BodyPatternStub,
};

pub struct JsonBodyRelaxedMatcher {
    value: Value,
    is_ignore_extra_elements: bool,
    is_ignore_array_order: bool,
}

impl JsonBodyRelaxedMatcher {
    fn match_relaxed(&self, req_body: &Value) -> bool {
        RelaxedValue(&self.value) == RelaxedValue(req_body)
    }

    fn match_ignoring_extra(&self, req_body: &Value) -> bool {
        RelaxedExtraJsonObj(&self.value) == RelaxedExtraJsonObj(req_body)
    }

    fn match_ignoring_array_order(&self, req_body: &Value) -> bool {
        RelaxedJsonArray(&self.value) == RelaxedJsonArray(req_body)
    }
}

impl Match for JsonBodyRelaxedMatcher {
    fn matches(&self, req: &Request) -> bool {
        deserialize::<Value>(&req.body)
            .ok()
            .as_ref()
            .map(|req_body| {
                if self.is_ignore_extra_elements && self.is_ignore_array_order {
                    self.match_relaxed(req_body)
                } else if self.is_ignore_extra_elements {
                    self.match_ignoring_extra(req_body)
                } else {
                    self.match_ignoring_array_order(req_body)
                }
            })
            .unwrap_or_default()
    }
}

impl TryFrom<&BodyPatternStub> for JsonBodyRelaxedMatcher {
    type Error = anyhow::Error;

    fn try_from(body: &BodyPatternStub) -> anyhow::Result<Self> {
        let is_ignore_extra_elements = body.is_ignore_extra_elements();
        let is_ignore_array_order = body.is_ignore_array_order();
        let is_relaxed = is_ignore_extra_elements || is_ignore_array_order;
        body.equal_to_json
            .as_ref()
            .filter(|_| is_relaxed && body.is_by_json_equality())
            .map(|v| Self {
                value: v.to_owned(),
                is_ignore_extra_elements,
                is_ignore_array_order,
            })
            .ok_or_else(|| anyhow::Error::msg(""))
    }
}
