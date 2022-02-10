use wiremock::{Match, Request};

use super::{JwtAuthStub, super::{AUTHORIZATION_HEADER, BEARER_PREFIX}};

pub struct JwtExactMatcher(String);

impl Match for JwtExactMatcher {
    fn matches(&self, req: &Request) -> bool {
        req.headers.get(&AUTHORIZATION_HEADER)
            .map(|v| v.as_str() == self.0.as_str())
            .unwrap_or_default()
    }
}

impl TryFrom<&JwtAuthStub> for JwtExactMatcher {
    type Error = anyhow::Error;

    fn try_from(stub: &JwtAuthStub) -> anyhow::Result<Self> {
        stub.equal_to.as_ref()
            .map(|eq| Self(format!("{} {}", BEARER_PREFIX, eq)))
            .ok_or_else(|| anyhow::Error::msg(""))
    }
}