use async_std::task::block_on;
use serde_json::Value;

use crate::model::request::body::BodyPatternStub;

use super::super::super::http::RecordedExchange;

impl From<&mut RecordedExchange> for Vec<BodyPatternStub> {
    fn from(ex: &mut RecordedExchange) -> Self {
        block_on(async move { ex.0.0.body_json::<Value>().await }).ok()
            .map(|json_body| BodyPatternStub { equal_to_json: Some(json_body), ..Default::default() })
            .map(|it| vec![it])
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod req_body_mapping_tests {
    use http_types::Request;
    use serde_json::json;

    use crate::record::http::RecordedRequest;

    use super::*;

    #[test]
    fn should_map_json_body() {
        let body = json!({"name": "beltram"});
        let mut req = Request::post("http://localhost");
        req.set_body(body.clone());
        let mut exchange = RecordedExchange { 0: RecordedRequest(req), ..Default::default() };
        let expected = BodyPatternStub { equal_to_json: Some(body), ..Default::default() };
        assert!(Vec::<BodyPatternStub>::from(&mut exchange).eq(&vec![expected]))
    }

    #[test]
    fn should_map_empty_body() {
        let body = json!({});
        let mut req = Request::post("http://localhost");
        req.set_body(body.clone());
        let mut exchange = RecordedExchange { 0: RecordedRequest(req), ..Default::default() };
        let expected = BodyPatternStub { equal_to_json: Some(body), ..Default::default() };
        assert!(Vec::<BodyPatternStub>::from(&mut exchange).eq(&vec![expected]))
    }

    #[test]
    fn should_map_missing_json_body() {
        let req = Request::post("http://localhost");
        let mut exchange = RecordedExchange { 0: RecordedRequest(req), ..Default::default() };
        assert!(Vec::<BodyPatternStub>::from(&mut exchange).is_empty())
    }
}
