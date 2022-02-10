use std::str::FromStr;

use jsonwebtoken::Algorithm;
use wiremock::{Match, Request};

use super::{JwtAlgStub, super::super::AUTHORIZATION_HEADER, super::super::BEARER_PREFIX};

pub struct JwtAlgExactMatcher(Algorithm);

impl Match for JwtAlgExactMatcher {
    fn matches(&self, req: &Request) -> bool {
        req.headers.get(&AUTHORIZATION_HEADER)
            .map(|v| v.as_str())
            .filter(|h| h.contains(BEARER_PREFIX))
            .map(|h| &h[BEARER_PREFIX.len() + 1..])
            .and_then(|jwt| jsonwebtoken::decode_header(jwt).ok())
            .map(|h| h.alg == self.0)
            .unwrap_or_default()
    }
}

impl TryFrom<&JwtAlgStub> for JwtAlgExactMatcher {
    type Error = anyhow::Error;

    fn try_from(stub: &JwtAlgStub) -> anyhow::Result<Self> {
        stub.equal_to.as_ref()
            .and_then(|eq| Algorithm::from_str(eq).ok())
            .map(Self)
            .ok_or_else(|| anyhow::Error::msg(""))
    }
}