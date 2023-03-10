use protobuf::reflect::MessageDescriptor;

use crate::{
    model::request::body::{eq_relaxed::JsonBodyRelaxedMatcher, BodyMatcherStub},
    wiremock::{Match, Request},
};

pub struct GrpcBodyRelaxedMatcher(JsonBodyRelaxedMatcher, MessageDescriptor);

impl GrpcBodyRelaxedMatcher {
    pub fn try_new(matcher: &BodyMatcherStub, md: MessageDescriptor) -> Option<Self> {
        JsonBodyRelaxedMatcher::try_from(matcher).ok().map(|m| Self(m, md))
    }
}

impl Match for GrpcBodyRelaxedMatcher {
    fn matches(&self, request: &Request) -> bool {
        super::proto_to_json_str(&request.body, &self.1)
            .map(|proto| self.0.matching_relaxed_json(proto.as_bytes()))
            .unwrap_or_default()
    }
}
