use crate::wiremock::{Match, Request};

use crate::model::request::body::{json_path_contains::JsonBodyPathContainsMatcher, BodyMatcherStub};
use protobuf::reflect::MessageDescriptor;

pub struct GrpcJsonPathContainsBodyMatcher(JsonBodyPathContainsMatcher, MessageDescriptor);

impl GrpcJsonPathContainsBodyMatcher {
    pub fn try_new(matcher: &BodyMatcherStub, md: MessageDescriptor) -> Option<Self> {
        JsonBodyPathContainsMatcher::try_from(matcher).ok().map(|m| Self(m, md))
    }
}

impl Match for GrpcJsonPathContainsBodyMatcher {
    fn matches(&self, request: &Request) -> bool {
        let proto = super::proto_to_json_str(&request.body, &self.1);
        self.0.matching_json_path_contains(proto.as_bytes())
    }
}
