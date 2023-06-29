use protobuf::reflect::MessageDescriptor;

use crate::{
    model::request::body::{eq::BodyExactMatcher, BodyMatcherStub},
    wiremock_rs::{Match, Request},
};

pub struct GrpcBodyExactMatcher(BodyExactMatcher, MessageDescriptor);

impl GrpcBodyExactMatcher {
    pub fn try_new(matcher: &BodyMatcherStub, md: MessageDescriptor) -> Option<Self> {
        BodyExactMatcher::try_from(matcher).ok().map(|m| Self(m, md))
    }
}

impl Match for GrpcBodyExactMatcher {
    fn matches(&self, request: &Request) -> bool {
        super::proto_to_json_str(&request.body, &self.1)
            .map(|proto| self.0.matching_exact_json(proto.as_bytes()))
            .unwrap_or_default()
    }
}
