use serde_json::{Map, Value};

use crate::model::request::{matcher::MatcherValueStub, query::HttpQueryParamsStub};

use super::super::super::RecordedExchange;

impl From<&mut RecordedExchange> for HttpQueryParamsStub {
    fn from(ex: &mut RecordedExchange) -> Self {
        let req = ex.req();
        let queries = req.url().query_pairs().into_iter()
            .map(|(k, v)| {
                let v = MatcherValueStub { equal_to: Some(v.into()), ..Default::default() };
                let v = serde_json::to_value(v).unwrap();
                (k.to_string(), v)
            })
            .collect::<Map<String, Value>>();
        if queries.is_empty() {
            Self { query_parameters: None }
        } else {
            Self { query_parameters: Some(queries) }
        }
    }
}

#[cfg(test)]
mod req_queries_mapping_tests {
    use http_types::Request;
    use serde_json::json;

    use crate::record::RecordedRequest;

    use super::*;

    #[test]
    fn should_map_single_query() {
        let req = Request::get("http://localhost?a=1");
        let mut exchange = RecordedExchange { 0: RecordedRequest(req), ..Default::default() };
        let expected = Map::from_iter(vec![("a".to_string(), json!({"equalTo": "1"}))]);
        assert_eq!(
            HttpQueryParamsStub::from(&mut exchange),
            HttpQueryParamsStub { query_parameters: Some(expected) }
        )
    }

    #[test]
    fn should_map_many_query() {
        let req = Request::get("http://localhost?a=1&b=2");
        let mut exchange = RecordedExchange { 0: RecordedRequest(req), ..Default::default() };
        let expected = Map::from_iter(vec![
            ("a".to_string(), json!({"equalTo": "1"})),
            ("b".to_string(), json!({"equalTo": "2"})),
        ]);
        assert_eq!(
            HttpQueryParamsStub::from(&mut exchange),
            HttpQueryParamsStub { query_parameters: Some(expected) }
        )
    }

    #[test]
    fn should_not_fail_when_no_queries() {
        let req = Request::get("http://localhost");
        let mut exchange = RecordedExchange { 0: RecordedRequest(req), ..Default::default() };
        assert_eq!(
            HttpQueryParamsStub::from(&mut exchange),
            HttpQueryParamsStub { query_parameters: None }
        )
    }

    #[test]
    fn should_map_multi_query() {
        let req = Request::get("http://localhost?a=1&a=2");
        let mut exchange = RecordedExchange { 0: RecordedRequest(req), ..Default::default() };
        let expected = Map::from_iter(vec![
            ("a".to_string(), json!({"equalTo": "1"})),
            ("a".to_string(), json!({"equalTo": "2"})),
        ]);
        assert_eq!(
            HttpQueryParamsStub::from(&mut exchange),
            HttpQueryParamsStub { query_parameters: Some(expected) }
        )
    }
}
