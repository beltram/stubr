use crate::wiremock_rs::{Match, Request};

use crate::model::request::body::{binary_eq::BinaryExactMatcher, BodyMatcherStub};
use protobuf::reflect::MessageDescriptor;

pub struct GrpcBinaryExactBodyMatcher(BinaryExactMatcher, MessageDescriptor);

impl GrpcBinaryExactBodyMatcher {
    pub fn try_new(matcher: &BodyMatcherStub, md: MessageDescriptor) -> Option<Self> {
        BinaryExactMatcher::try_from(matcher).ok().map(|m| Self(m, md))
    }
}

impl Match for GrpcBinaryExactBodyMatcher {
    fn matches(&self, request: &Request) -> bool {
        self.0.matching_binary(&request.body[5..])
    }
}
