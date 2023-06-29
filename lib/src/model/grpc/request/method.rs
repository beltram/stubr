use crate::wiremock_rs::{Match, Request};
use crate::{StubrError, StubrResult};

pub struct GrpcMethodMatcher(regex::Regex);

impl GrpcMethodMatcher {
    pub fn try_new(path: &str) -> StubrResult<Self> {
        Ok(Self(regex::Regex::new(path)?))
    }
}

pub struct GrpcSvcMatcher(regex::Regex);

impl GrpcSvcMatcher {
    pub fn try_new(path: &str) -> StubrResult<Self> {
        Ok(Self(regex::Regex::new(path)?))
    }
}

pub(crate) struct GrpcMethod<'a>(pub(crate) &'a str);

impl<'a> From<&'a str> for GrpcMethod<'a> {
    fn from(value: &'a str) -> Self {
        Self(value)
    }
}

pub(crate) struct GrpcSvc<'a>(pub(crate) &'a str);

impl<'a> TryFrom<&'a str> for GrpcSvc<'a> {
    type Error = StubrError;

    fn try_from(value: &'a str) -> StubrResult<Self> {
        let svc = value.split('.').last().ok_or(StubrError::InvalidGrpcRequest)?;
        Ok(Self(svc))
    }
}

impl Match for GrpcMethodMatcher {
    fn matches(&self, request: &Request) -> bool {
        parse_path(request)
            .map(|(method, _)| self.0.is_match(method.0))
            .unwrap_or_default()
    }
}

impl Match for GrpcSvcMatcher {
    fn matches(&self, request: &Request) -> bool {
        parse_path(request).map(|(_, svc)| self.0.is_match(svc.0)).unwrap_or_default()
    }
}

pub(crate) fn parse_path(request: &Request) -> StubrResult<(GrpcMethod, GrpcSvc)> {
    let mut paths = request.url.path_segments().ok_or(StubrError::InvalidGrpcRequest)?;
    let svc: GrpcSvc = paths.next().ok_or(StubrError::InvalidGrpcRequest)?.try_into()?;
    let method: GrpcMethod = paths.next().ok_or(StubrError::InvalidGrpcRequest)?.into();
    Ok((method, svc))
}
