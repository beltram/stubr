use crate::record::RecordInput;

use super::super::model::{JsonStub, request::RequestStub, response::ResponseStub};

pub mod req;
pub mod resp;

impl From<RecordInput<'_>> for JsonStub {
    fn from((ex, cfg): RecordInput) -> Self {
        Self {
            uuid: None,
            request: RequestStub::from((&mut *ex, cfg.clone())),
            response: ResponseStub::from((&mut *ex, cfg)),
        }
    }
}
