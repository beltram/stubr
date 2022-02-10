use jsonwebtoken::Algorithm;
use wiremock::{Match, Request};

use super::{JwtAlgStub, super::super::helpers::RequestAuthExtension};

pub struct JwtAlgOneOfMatcher(Vec<Algorithm>);

impl Match for JwtAlgOneOfMatcher {
    fn matches(&self, req: &Request) -> bool {
        req.jwt_header()
            .map(|h| self.0.contains(&h.alg))
            .unwrap_or_default()
    }
}

impl TryFrom<&JwtAlgStub> for JwtAlgOneOfMatcher {
    type Error = anyhow::Error;

    fn try_from(stub: &JwtAlgStub) -> anyhow::Result<Self> {
        stub.one_of.as_ref()
            .map(|o| Self(o.to_owned()))
            .ok_or_else(|| anyhow::Error::msg(""))
    }
}