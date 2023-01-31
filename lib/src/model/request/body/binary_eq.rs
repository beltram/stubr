use crate::wiremock::{Match, Request};

use super::BodyMatcherStub;

pub struct BinaryExactMatcher(Vec<u8>);

impl Match for BinaryExactMatcher {
    fn matches(&self, req: &Request) -> bool {
        self.matching_binary(&req.body)
    }
}

impl BinaryExactMatcher {
    pub fn matching_binary(&self, bytes: &[u8]) -> bool {
        self.0 == bytes
    }
}

impl TryFrom<&BodyMatcherStub> for BinaryExactMatcher {
    type Error = anyhow::Error;

    fn try_from(body: &BodyMatcherStub) -> anyhow::Result<Self> {
        use base64::Engine as _;
        body.binary_equal_to
            .as_ref()
            .filter(|_| body.is_by_binary_equality())
            .and_then(|it| base64::prelude::BASE64_STANDARD.decode(it).ok())
            .map(Self)
            .ok_or_else(|| anyhow::Error::msg(""))
    }
}
