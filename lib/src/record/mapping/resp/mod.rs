use crate::{model::response::{body::BodyStub, headers::HttpRespHeadersStub, ResponseStub}, record::RecordInput};

pub mod body;
pub mod headers;

impl From<RecordInput<'_>> for ResponseStub {
    fn from((ex, cfg): RecordInput) -> Self {
        Self {
            status: Some(ex.resp().status().into()),
            fixed_delay_milliseconds: None,
            body: BodyStub::from(&mut *ex),
            headers: HttpRespHeadersStub::from((&mut *ex, cfg)),
            transformers: vec![],
        }
    }
}

#[cfg(test)]
mod status_mapping_tests {
    use http_types::Response;

    use crate::{record::http::{RecordedExchange, RecordedResponse}, RecordConfig};

    use super::*;

    #[test]
    fn should_map_200_status() {
        let resp = Response::new(200);
        let mut exchange = RecordedExchange { 1: RecordedResponse(resp), ..Default::default() };
        assert_eq!(ResponseStub::from((&mut exchange, RecordConfig::default())).status, Some(200))
    }

    #[test]
    fn should_map_400_status() {
        let resp = Response::new(400);
        let mut exchange = RecordedExchange { 1: RecordedResponse(resp), ..Default::default() };
        assert_eq!(ResponseStub::from((&mut exchange, RecordConfig::default())).status, Some(400))
    }

    #[test]
    fn should_map_500_status() {
        let resp = Response::new(500);
        let mut exchange = RecordedExchange { 1: RecordedResponse(resp), ..Default::default() };
        assert_eq!(ResponseStub::from((&mut exchange, RecordConfig::default())).status, Some(500))
    }
}