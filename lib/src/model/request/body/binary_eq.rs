use std::convert::TryFrom;

use wiremock::{Match, Request};

use super::BodyPatternDto;

pub struct BinaryEqualMatcher(Vec<u8>);

impl Match for BinaryEqualMatcher {
    fn matches(&self, req: &Request) -> bool {
        self.0 == req.body
    }
}

impl TryFrom<&BodyPatternDto> for BinaryEqualMatcher {
    type Error = anyhow::Error;

    fn try_from(body: &BodyPatternDto) -> anyhow::Result<Self> {
        body.binary_equal_to.as_ref()
            .filter(|_| body.is_by_binary_equality())
            .and_then(|it| base64::decode(it).ok())
            .map(|it| BinaryEqualMatcher(it))
            .ok_or_else(|| anyhow::Error::msg("No body matcher by binary equality"))
    }
}