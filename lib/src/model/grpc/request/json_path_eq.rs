use crate::wiremock::{Match, Request};

use crate::model::request::body::{json_path_eq::JsonBodyPathEqMatcher, BodyMatcherStub};
use protobuf::reflect::MessageDescriptor;

pub struct GrpcJsonPathEqBodyMatcher(JsonBodyPathEqMatcher, MessageDescriptor);

impl GrpcJsonPathEqBodyMatcher {
    pub fn try_new(matcher: &BodyMatcherStub, md: MessageDescriptor) -> Option<Self> {
        JsonBodyPathEqMatcher::try_from(matcher).ok().map(|m| Self(m, md))
    }
}

impl Match for GrpcJsonPathEqBodyMatcher {
    fn matches(&self, request: &Request) -> bool {
        let proto = super::proto_to_json_str(&request.body, &self.1);
        self.0.matching_json_path_eq(proto.as_bytes())
    }
}
