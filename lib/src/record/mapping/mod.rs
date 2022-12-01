use crate::record::RecordInput;

use super::super::model::{request::RequestStub, response::ResponseStub, JsonStub};

pub mod req;
pub mod resp;

impl From<RecordInput<'_>> for JsonStub {
    fn from((ex, cfg): RecordInput) -> Self {
        Self {
            id: None,
            uuid: None,
            priority: None,
            expect: None,
            request: RequestStub::from((&mut *ex, cfg)),
            response: ResponseStub::from((&mut *ex, cfg)),
        }
    }
}
