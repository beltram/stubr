use std::convert::TryFrom;

use wiremock::matchers::{body_json, BodyExactMatcher};

use super::BodyPatternDto;

impl TryFrom<&BodyPatternDto> for BodyExactMatcher {
    type Error = anyhow::Error;

    fn try_from(body: &BodyPatternDto) -> anyhow::Result<Self> {
        body.equal_to_json.as_ref()
            .filter(|_| body.is_by_json_equality() && !body.is_ignore_extra_elements())
            .map(body_json)
            .ok_or_else(|| anyhow::Error::msg("No body matcher by json equality"))
    }
}