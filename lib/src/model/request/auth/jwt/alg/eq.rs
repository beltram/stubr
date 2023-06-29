use crate::error::StubrResult;
use crate::wiremock_rs::{Match, Request};
use jsonwebtoken::Algorithm;

use crate::model::request::auth::helpers::RequestAuthExtension;
use crate::StubrError;

use super::JwtAlgStub;

pub struct JwtAlgExactMatcher(Algorithm);

impl Match for JwtAlgExactMatcher {
    fn matches(&self, req: &Request) -> bool {
        req.jwt_header().map(|h| h.alg == self.0).unwrap_or_default()
    }
}

impl TryFrom<&JwtAlgStub> for JwtAlgExactMatcher {
    type Error = StubrError;

    fn try_from(stub: &JwtAlgStub) -> StubrResult<Self> {
        stub.equal_to
            .as_ref()
            .map(|eq| eq.parse())
            .transpose()?
            .map(Self)
            .ok_or(StubrError::QuietError)
    }
}
