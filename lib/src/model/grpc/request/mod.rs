use std::{hash::Hash, path::PathBuf};

use protobuf::reflect::MessageDescriptor;

use crate::{
    model::{
        grpc::proto::parse_message_descriptor,
        request::{
            body::BodyMatcherStub,
            method::{HttpMethodStub, Verb},
        },
    },
    wiremock::MockBuilder,
    StubrError, StubrResult,
};

pub mod binary_eq;
pub mod eq;
pub mod eq_relaxed;
pub mod json_path;
pub mod json_path_contains;
pub mod json_path_eq;
pub mod method;

#[derive(Debug, Clone, Hash, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct GrpcRequestStub {
    /// Name of the message definition within protobuf
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Name of the gRPC method
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// Name of the gRPC service
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// request body matchers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_patterns: Option<Vec<BodyMatcherStub>>,
}

impl GrpcRequestStub {
    pub fn try_new(request: &GrpcRequestStub, proto_file: Option<&PathBuf>) -> StubrResult<MockBuilder> {
        let mut mock = MockBuilder::from(&HttpMethodStub(Verb::Post));
        if let Some(method) = request.method.as_ref() {
            mock = mock.and(method::GrpcMethodMatcher::try_new(method)?);
        }
        if let Some(svc) = request.service.as_ref() {
            mock = mock.and(method::GrpcSvcMatcher::try_new(svc)?);
        }
        if let Some(matchers) = request.body_patterns.as_ref() {
            let proto_file = proto_file.ok_or(StubrError::MissingProtoFile)?;
            let md = request.descriptor(proto_file)?;
            for matcher in matchers {
                if let Some(exact_json) = eq::GrpcBodyExactMatcher::try_new(matcher, md.clone()) {
                    mock = mock.and(exact_json)
                }
                if let Some(relaxed_json) = eq_relaxed::GrpcBodyRelaxedMatcher::try_new(matcher, md.clone()) {
                    mock = mock.and(relaxed_json)
                }
                if let Some(json_path) = json_path::GrpcJsonPathBodyMatcher::try_new(matcher, md.clone()) {
                    mock = mock.and(json_path)
                }
                if let Some(json_path_contains) = json_path_contains::GrpcJsonPathContainsBodyMatcher::try_new(matcher, md.clone()) {
                    mock = mock.and(json_path_contains)
                }
                if let Some(json_path_eq) = json_path_eq::GrpcJsonPathEqBodyMatcher::try_new(matcher, md.clone()) {
                    mock = mock.and(json_path_eq)
                }
                if let Some(binary_eq) = binary_eq::GrpcBinaryExactBodyMatcher::try_new(matcher, md.clone()) {
                    mock = mock.and(binary_eq)
                }
            }
        }
        Ok(mock)
    }

    pub fn descriptor(&self, proto_file: &PathBuf) -> StubrResult<MessageDescriptor> {
        let msg = self.message.as_ref().ok_or(StubrError::MissingProtoMessage)?;
        parse_message_descriptor(msg, proto_file)
    }
}

pub fn proto_to_json_str(message: &[u8], md: &MessageDescriptor) -> StubrResult<String> {
    let body = &message[5..];
    let message = md.parse_from_bytes(body)?;
    let options = protobuf_json_mapping::PrintOptions {
        enum_values_int: false,
        proto_field_name: true,
        always_output_default_values: false,
        ..Default::default()
    };
    Ok(protobuf_json_mapping::print_to_string_with_options(&*message, &options)?)
}
