use crate::model::request::url::HttpUrlStub;

use super::super::super::http::RecordedExchange;

impl From<&mut RecordedExchange> for HttpUrlStub {
    fn from(ex: &mut RecordedExchange) -> Self {
        let path = ex.req().url().path().trim().to_string();
        let url = if !path.is_empty() && path != "/" { Some(path) } else { None };
        Self { url_path: url, ..Default::default() }
    }
}

#[cfg(test)]
mod req_url_mapping_tests {
    use http_types::Request;

    use crate::record::http::RecordedRequest;

    use super::*;

    #[test]
    fn should_map_path_to_url() {
        let path = "http://localhost/api/pets";
        let req = RecordedRequest(Request::get(path));
        let mut exchange = RecordedExchange { 0: req, ..Default::default() };
        assert_eq!(HttpUrlStub::from(&mut exchange), HttpUrlStub { url_path: Some("/api/pets".to_string()), ..Default::default() })
    }

    #[test]
    fn should_not_map_empty_path() {
        let req = RecordedRequest(Request::get("http://localhost"));
        let mut exchange = RecordedExchange { 0: req, ..Default::default() };
        assert_eq!(HttpUrlStub::from(&mut exchange), HttpUrlStub { url_path: None, ..Default::default() })
    }

    #[test]
    fn should_not_map_blank_path() {
        let req = RecordedRequest(Request::get("http://localhost  "));
        let mut exchange = RecordedExchange { 0: req, ..Default::default() };
        assert_eq!(HttpUrlStub::from(&mut exchange), HttpUrlStub { url_path: None, ..Default::default() })
    }

    #[test]
    fn should_not_map_single_slash_path() {
        let req = RecordedRequest(Request::get("http://localhost/"));
        let mut exchange = RecordedExchange { 0: req, ..Default::default() };
        assert_eq!(HttpUrlStub::from(&mut exchange), HttpUrlStub { url_path: None, ..Default::default() })
    }
}
