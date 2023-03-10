use itertools::Itertools;
use serde_json::{Map, Value};

use crate::{model::response::headers::HttpRespHeadersStub, record::RecordInput};

impl From<RecordInput<'_>> for HttpRespHeadersStub {
    fn from((ex, cfg): RecordInput) -> Self {
        let resp = ex.resp();
        let headers = resp
            .header_names()
            .sorted_by(|a, b| Ord::cmp(a.as_str(), b.as_str()))
            .filter(|k| {
                !cfg.except_response_headers
                    .as_ref()
                    .map(|it| it.contains(&k.as_str()))
                    .unwrap_or_default()
            })
            .filter_map(|k| resp.header(k).map(|v| (k, v)))
            .map(|(k, v)| {
                let k = k.to_string();
                let v: Value = v.iter().map(|it| it.as_str()).join(", ").into();
                (k, v)
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
mod resp_header_mapping_tests {
    use http_types::Response;
    use serde_json::{json, Map};

    use crate::{
        record::{RecordedExchange, RecordedResponse},
        RecordConfig,
    };

    use super::*;

    #[test]
    fn should_map_single_header() {
        let mut resp = Response::new(200);
        resp.append_header("x-key", "value");
        let mut exchange = RecordedExchange {
            1: RecordedResponse(resp),
            ..Default::default()
        };
        let expected = Map::from_iter(vec![("x-key".to_string(), json!("value"))]);
        assert_eq!(
            HttpRespHeadersStub::from((&mut exchange, &RecordConfig::default())),
            HttpRespHeadersStub { headers: Some(expected) }
        )
    }

    #[test]
    fn should_map_many_headers() {
        let mut resp = Response::new(200);
        resp.append_header("x-a", "value-a");
        resp.append_header("x-b", "value-b");
        let mut exchange = RecordedExchange {
            1: RecordedResponse(resp),
            ..Default::default()
        };
        let expected = Map::from_iter(vec![
            ("x-a".to_string(), json!("value-a")),
            ("x-b".to_string(), json!("value-b")),
        ]);
        assert_eq!(
            HttpRespHeadersStub::from((&mut exchange, &RecordConfig::default())),
            HttpRespHeadersStub { headers: Some(expected) }
        )
    }

    #[test]
    fn should_not_fail_when_no_header() {
        let resp = Response::new(200);
        let mut exchange = RecordedExchange {
            1: RecordedResponse(resp),
            ..Default::default()
        };
        assert_eq!(
            HttpRespHeadersStub::from((&mut exchange, &RecordConfig::default())),
            HttpRespHeadersStub { headers: None }
        )
    }

    #[test]
    fn should_map_multi_header() {
        let mut resp = Response::new(200);
        resp.append_header("cache-control", "no-cache, no-transform");
        let mut exchange = RecordedExchange {
            1: RecordedResponse(resp),
            ..Default::default()
        };
        let expected = Map::from_iter(vec![("cache-control".to_string(), json!("no-cache, no-transform"))]);
        assert_eq!(
            HttpRespHeadersStub::from((&mut exchange, &RecordConfig::default())),
            HttpRespHeadersStub { headers: Some(expected) }
        )
    }

    #[test]
    fn should_ignore_excluded_headers() {
        let mut resp = Response::new(200);
        resp.append_header("x-a", "a");
        resp.append_header("x-b", "b");
        let mut exchange = RecordedExchange {
            1: RecordedResponse(resp),
            ..Default::default()
        };
        let expected = Map::from_iter(vec![("x-b".to_string(), json!("b"))]);
        let cfg = RecordConfig {
            except_response_headers: Some(vec!["x-a"]),
            ..Default::default()
        };
        assert_eq!(
            HttpRespHeadersStub::from((&mut exchange, &cfg)),
            HttpRespHeadersStub { headers: Some(expected) }
        )
    }
}
