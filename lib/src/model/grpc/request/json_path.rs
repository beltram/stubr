use protobuf::reflect::MessageDescriptor;

use crate::{
    model::request::body::{json_path::JsonPathBodyMatcher, BodyMatcherStub},
    wiremock::{Match, Request},
};

pub struct GrpcJsonPathBodyMatcher(JsonPathBodyMatcher, MessageDescriptor);

impl GrpcJsonPathBodyMatcher {
    pub fn try_new(matcher: &BodyMatcherStub, md: MessageDescriptor) -> Option<Self> {
        JsonPathBodyMatcher::try_from(matcher).ok().map(|m| Self(m, md))
    }
}

impl Match for GrpcJsonPathBodyMatcher {
    fn matches(&self, request: &Request) -> bool {
        super::proto_to_json_str(&request.body, &self.1)
            .map(|proto| self.0.matching_json_path(proto.as_bytes()))
            .unwrap_or_default()
    }
}
