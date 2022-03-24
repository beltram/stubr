use itertools::Itertools;
use serde_json::{Map, Value};

use crate::{
    model::request::{headers::HttpReqHeadersStub, matcher::MatcherValueStub},
    record::RecordInput,
};

impl From<RecordInput<'_>> for HttpReqHeadersStub {
    fn from((ex, cfg): RecordInput) -> Self {
        let req = ex.req();
        let headers = req.header_names().into_iter()
            .sorted_by(|a, b| Ord::cmp(a.as_str(), b.as_str()))
            .filter(|k| {
                !cfg.except_request_headers.as_ref()
                    .map(|it| it.contains(&k.as_str()))
                    .unwrap_or_default()
            })
            .filter_map(|k| req.header(k).map(|v| (k, v)))
            .map(|(k, v)| {
                let v: Value = v.iter().map(|it| it.as_str()).join(", ").into();
                let v = MatcherValueStub { equal_to: Some(v), ..Default::default() };
                let v = serde_json::to_value(v).unwrap();
                (k.to_string(), v)
            })
            .collect::<Map<String, Value>>();
        if headers.is_empty() {
            Self { headers: None }
        } else {
            Self { headers: Some(headers) }
        }
    }
}

#[cfg(test)]
mod req_url_mapping_tests {
    use http_types::Request;
    use serde_json::{json, Map};

    use crate::{record::{RecordedExchange, RecordedRequest}, RecordConfig};

    use super::*;

    #[test]
    fn should_map_single_header() {
        let mut req = Request::get("http://localhost");
        req.append_header("x-key", "value");
        let mut exchange = RecordedExchange { 0: RecordedRequest(req), ..Default::default() };
        let expected = Map::from_iter(vec![("x-key".to_string(), json!({"equalTo": "value"}))]);
        assert_eq!(
            HttpReqHeadersStub::from((&mut exchange, &RecordConfig::default())),
            HttpReqHeadersStub { headers: Some(expected) }
        )
    }

    #[test]
    fn should_map_many_headers() {
        let mut req = Request::get("http://localhost");
        req.append_header("x-a", "value-a");
        req.append_header("x-b", "value-b");
        let mut exchange = RecordedExchange { 0: RecordedRequest(req), ..Default::default() };
        let expected = Map::from_iter(vec![
            ("x-a".to_string(), json!({"equalTo": "value-a"})),
            ("x-b".to_string(), json!({"equalTo": "value-b"})),
        ]);
        assert_eq!(
            HttpReqHeadersStub::from((&mut exchange, &RecordConfig::default())),
            HttpReqHeadersStub { headers: Some(expected) }
        )
    }

    #[test]
    fn should_not_fail_when_no_header() {
        let req = Request::get("http://localhost");
        let mut exchange = RecordedExchange { 0: RecordedRequest(req), ..Default::default() };
        assert_eq!(
            HttpReqHeadersStub::from((&mut exchange, &RecordConfig::default())),
            HttpReqHeadersStub { headers: None }
        )
    }

    #[test]
    fn should_map_multi_header() {
        let mut req = Request::get("http://localhost");
        req.append_header("cache-control", "no-cache, no-transform");
        let mut exchange = RecordedExchange { 0: RecordedRequest(req), ..Default::default() };
        let expected = Map::from_iter(vec![("cache-control".to_string(), json!({"equalTo": "no-cache, no-transform"}))]);
        assert_eq!(
            HttpReqHeadersStub::from((&mut exchange, &RecordConfig::default())),
            HttpReqHeadersStub { headers: Some(expected) }
        )
    }

    #[test]
    fn should_ignore_excluded_header() {
        let mut req = Request::get("http://localhost");
        req.append_header("x-a", "a");
        req.append_header("x-b", "b");
        let mut exchange = RecordedExchange { 0: RecordedRequest(req), ..Default::default() };
        let expected = Map::from_iter(vec![("x-b".to_string(), json!({"equalTo": "b"}))]);
        let cfg = RecordConfig { except_request_headers: Some(vec!["x-a"]), ..Default::default() };
        assert_eq!(
            HttpReqHeadersStub::from((&mut exchange, &cfg)),
            HttpReqHeadersStub { headers: Some(expected) }
        )
    }
}
