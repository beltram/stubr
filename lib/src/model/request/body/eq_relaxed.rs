use crate::error::StubrResult;
use crate::wiremock_rs::{Match, Request};
use crate::StubrError;
use serde_json::Value;

use super::{
    diff::{all::RelaxedValue, array::RelaxedJsonArray, extra::RelaxedExtraJsonObj},
    BodyMatcherStub,
};

pub struct JsonBodyRelaxedMatcher {
    value: Value,
    is_ignore_extra_elements: bool,
    is_ignore_array_order: bool,
}

impl Match for JsonBodyRelaxedMatcher {
    fn matches(&self, req: &Request) -> bool {
        self.matching_relaxed_json(&req.body)
    }
}

impl JsonBodyRelaxedMatcher {
    pub fn matching_relaxed_json(&self, bytes: &[u8]) -> bool {
        serde_json::from_slice::<Value>(bytes)
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

impl TryFrom<&BodyMatcherStub> for JsonBodyRelaxedMatcher {
    type Error = StubrError;

    fn try_from(body: &BodyMatcherStub) -> StubrResult<Self> {
        body.equal_to_json
            .as_ref()
            .filter(|_| body.is_relaxed_matching())
            .map(|v| Self {
                value: v.to_owned(),
                is_ignore_extra_elements: body.is_ignore_extra_elements(),
                is_ignore_array_order: body.is_ignore_array_order(),
            })
            .ok_or_else(|| StubrError::QuietError)
    }
}
