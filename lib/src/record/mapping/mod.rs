use crate::{
    model::{request::RequestStub, response::ResponseStub, JsonStub},
    record::RecordInput,
};

pub mod req;
pub mod resp;

impl From<RecordInput<'_>> for JsonStub {
    fn from((ex, cfg): RecordInput) -> Self {
        Self {
            id: None,
            uuid: None,
            priority: None,
            expect: None,
            http_request: Some(RequestStub::from((&mut *ex, cfg))),
            http_response: Some(ResponseStub::from((&mut *ex, cfg))),
            #[cfg(feature = "grpc")]
            grpc_request: None,
            #[cfg(feature = "grpc")]
            grpc_response: None,
            #[cfg(feature = "grpc")]
            proto_file: None,
        }
    }
}
