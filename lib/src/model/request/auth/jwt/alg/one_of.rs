use crate::error::StubrResult;
use crate::wiremock_rs::{Match, Request};
use crate::StubrError;
use jsonwebtoken::Algorithm;

use super::{super::super::helpers::RequestAuthExtension, JwtAlgStub};

pub struct JwtAlgOneOfMatcher(Vec<Algorithm>);

impl Match for JwtAlgOneOfMatcher {
    fn matches(&self, req: &Request) -> bool {
        req.jwt_header().map(|h| self.0.contains(&h.alg)).unwrap_or_default()
    }
}

impl TryFrom<&JwtAlgStub> for JwtAlgOneOfMatcher {
    type Error = StubrError;

    fn try_from(stub: &JwtAlgStub) -> StubrResult<Self> {
        stub.one_of
            .as_ref()
            .map(|o| Self(o.clone()))
            .ok_or_else(|| StubrError::QuietError)
    }
}
