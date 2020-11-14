use wiremock::matchers::{body_json, BodyExactMatcher};

use crate::model::request::body::BodyPatternDto;

impl From<&BodyPatternDto> for Option<BodyExactMatcher> {
    fn from(body: &BodyPatternDto) -> Self {
        body.equal_to_json.as_ref().map(body_json)
    }
}