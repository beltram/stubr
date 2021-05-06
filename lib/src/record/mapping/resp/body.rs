use async_std::task::block_on;
use serde_json::Value;

use crate::model::response::body::BodyStub;

use super::super::super::http::RecordedExchange;

impl From<&mut RecordedExchange> for BodyStub {
    fn from(ex: &mut RecordedExchange) -> Self {
        let json_body = block_on(async move {
            ex.1.0.take_body()
                .into_bytes().await.ok()
                .and_then(|b| {
                    ex.1.0.set_body(b.clone());
                    serde_json::from_slice::<Value>(b.as_slice()).ok()
                })
        });
        Self { json_body, ..Default::default() }
    }
}

#[cfg(test)]
mod resp_body_mapping_tests {
    use http_types::Response;
    use serde_json::json;

    use crate::record::http::RecordedResponse;

    use super::*;

    #[test]
    fn should_map_json_body() {
        let body = json!({"name": "beltram"});
        let mut resp = Response::new(200);
        resp.set_body(body.clone());
        let mut exchange = RecordedExchange { 1: RecordedResponse(resp), ..Default::default() };
        let expected = BodyStub { json_body: Some(body), ..Default::default() };
        assert_eq!(BodyStub::from(&mut exchange), expected)
    }

    #[test]
    fn should_map_empty_body() {
        let body = json!({});
        let mut resp = Response::new(200);
        resp.set_body(body.clone());
        let mut exchange = RecordedExchange { 1: RecordedResponse(resp), ..Default::default() };
        let expected = BodyStub { json_body: Some(body), ..Default::default() };
        assert_eq!(BodyStub::from(&mut exchange), expected)
    }

    #[test]
    fn should_map_missing_json_body() {
        let resp = Response::new(200);
        let mut exchange = RecordedExchange { 1: RecordedResponse(resp), ..Default::default() };
        let expected = BodyStub { json_body: None, ..Default::default() };
        assert_eq!(BodyStub::from(&mut exchange), expected)
    }
}