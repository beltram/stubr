use wiremock::matchers::{body_json, BodyExactMatcher};

use super::BodyPatternStub;

impl TryFrom<&BodyPatternStub> for BodyExactMatcher {
    type Error = anyhow::Error;

    fn try_from(body: &BodyPatternStub) -> anyhow::Result<Self> {
        let is_exact_matching = body.is_by_json_equality()
            && !body.is_ignore_extra_elements()
            && !body.is_ignore_array_order();
        body.equal_to_json.as_ref()
            .filter(|_| is_exact_matching)
            .map(body_json)
            .ok_or_else(|| anyhow::Error::msg(""))
    }
}