use crate::model::request::{
    body::BodyPatternStub,
    headers::HttpReqHeadersStub,
    method::HttpMethodStub,
    query::HttpQueryParamsStub,
    RequestStub,
    url::HttpUrlStub,
};
use crate::record::RecordInput;

pub mod method;
pub mod url;
pub mod headers;
pub mod queries;
pub mod body;

impl From<RecordInput<'_>> for RequestStub {
    fn from((ex, cfg): RecordInput) -> Self {
        Self {
            method: HttpMethodStub::from(&mut *ex),
            url: HttpUrlStub::from(&mut *ex),
            headers: HttpReqHeadersStub::from((&mut *ex, cfg)),
            queries: HttpQueryParamsStub::from(&mut *ex),
            body_patterns: Vec::<BodyPatternStub>::from(&mut *ex),
            ..Default::default()
        }
    }
}