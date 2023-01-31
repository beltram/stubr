use crate::wiremock::{Match, Request};

use crate::model::request::body::{eq_relaxed::JsonBodyRelaxedMatcher, BodyMatcherStub};
use protobuf::reflect::MessageDescriptor;

pub struct GrpcBodyRelaxedMatcher(JsonBodyRelaxedMatcher, MessageDescriptor);

impl GrpcBodyRelaxedMatcher {
    pub fn try_new(matcher: &BodyMatcherStub, md: MessageDescriptor) -> Option<Self> {
        JsonBodyRelaxedMatcher::try_from(matcher).ok().map(|m| Self(m, md))
    }
}

impl Match for GrpcBodyRelaxedMatcher {
    fn matches(&self, request: &Request) -> bool {
        let proto = super::proto_to_json_str(&request.body, &self.1);
        self.0.matching_relaxed_json(proto.as_bytes())
    }
}
