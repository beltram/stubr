use crate::wiremock::{Match, Request};

use crate::model::request::body::{json_path::JsonPathBodyMatcher, BodyMatcherStub};
use protobuf::reflect::MessageDescriptor;

pub struct GrpcJsonPathBodyMatcher(JsonPathBodyMatcher, MessageDescriptor);

impl GrpcJsonPathBodyMatcher {
    pub fn try_new(matcher: &BodyMatcherStub, md: MessageDescriptor) -> Option<Self> {
        JsonPathBodyMatcher::try_from(matcher).ok().map(|m| Self(m, md))
    }
}

impl Match for GrpcJsonPathBodyMatcher {
    fn matches(&self, request: &Request) -> bool {
        let proto = super::proto_to_json_str(&request.body, &self.1);
        self.0.matching_json_path(proto.as_bytes())
    }
}
