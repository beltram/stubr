use protobuf::reflect::MessageDescriptor;

use crate::{
    model::request::body::{json_path_contains::JsonBodyPathContainsMatcher, BodyMatcherStub},
    wiremock::{Match, Request}
};

pub struct GrpcJsonPathContainsBodyMatcher(JsonBodyPathContainsMatcher, MessageDescriptor);

impl GrpcJsonPathContainsBodyMatcher {
    pub fn try_new(matcher: &BodyMatcherStub, md: MessageDescriptor) -> Option<Self> {
        JsonBodyPathContainsMatcher::try_from(matcher).ok().map(|m| Self(m, md))
    }
}

impl Match for GrpcJsonPathContainsBodyMatcher {
    fn matches(&self, request: &Request) -> bool {
        super::proto_to_json_str(&request.body, &self.1)
            .map(|proto| self.0.matching_json_path_contains(proto.as_bytes()))
            .unwrap_or_default()
    }
}
