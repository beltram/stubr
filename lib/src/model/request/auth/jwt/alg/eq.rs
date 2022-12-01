use std::str::FromStr;

use jsonwebtoken::Algorithm;
use wiremock::{Match, Request};

use crate::model::request::auth::helpers::RequestAuthExtension;

use super::JwtAlgStub;

pub struct JwtAlgExactMatcher(Algorithm);

impl Match for JwtAlgExactMatcher {
    fn matches(&self, req: &Request) -> bool {
        req.jwt_header().map(|h| h.alg == self.0).unwrap_or_default()
    }
}

impl TryFrom<&JwtAlgStub> for JwtAlgExactMatcher {
    type Error = anyhow::Error;

    fn try_from(stub: &JwtAlgStub) -> anyhow::Result<Self> {
        stub.equal_to
            .as_ref()
            .and_then(|eq| Algorithm::from_str(eq).ok())
            .map(Self)
            .ok_or_else(|| anyhow::Error::msg(""))
    }
}
