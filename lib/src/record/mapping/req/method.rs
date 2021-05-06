use crate::model::request::method::HttpMethodStub;

use super::super::super::http::RecordedExchange;

impl From<&mut RecordedExchange> for HttpMethodStub {
    fn from(ex: &mut RecordedExchange) -> Self {
        Self::from(ex.req().method().to_string().to_uppercase().as_str())
    }
}

#[cfg(test)]
mod req_method_mapping_tests {
    use http_types::Request;

    use crate::record::http::RecordedRequest;

    use super::*;

    #[test]
    fn should_map_get() {
        let req = RecordedRequest(Request::get("http://localhost"));
        let mut exchange = RecordedExchange { 0: req, ..Default::default() };
        assert_eq!(HttpMethodStub::from(&mut exchange), HttpMethodStub::from("GET"))
    }

    #[test]
    fn should_map_post() {
        let req = RecordedRequest(Request::post("http://localhost"));
        let mut exchange = RecordedExchange { 0: req, ..Default::default() };
        assert_eq!(HttpMethodStub::from(&mut exchange), HttpMethodStub::from("POST"))
    }

    #[test]
    fn should_map_put() {
        let req = RecordedRequest(Request::put("http://localhost"));
        let mut exchange = RecordedExchange { 0: req, ..Default::default() };
        assert_eq!(HttpMethodStub::from(&mut exchange), HttpMethodStub::from("PUT"))
    }

    #[test]
    fn should_map_delete() {
        let req = RecordedRequest(Request::delete("http://localhost"));
        let mut exchange = RecordedExchange { 0: req, ..Default::default() };
        assert_eq!(HttpMethodStub::from(&mut exchange), HttpMethodStub::from("DELETE"))
    }

    #[test]
    fn should_map_patch() {
        let req = RecordedRequest(Request::patch("http://localhost"));
        let mut exchange = RecordedExchange { 0: req, ..Default::default() };
        assert_eq!(HttpMethodStub::from(&mut exchange), HttpMethodStub::from("PATCH"))
    }

    #[test]
    fn should_map_head() {
        let req = RecordedRequest(Request::head("http://localhost"));
        let mut exchange = RecordedExchange { 0: req, ..Default::default() };
        assert_eq!(HttpMethodStub::from(&mut exchange), HttpMethodStub::from("HEAD"))
    }

    #[test]
    fn should_map_options() {
        let req = RecordedRequest(Request::options("http://localhost"));
        let mut exchange = RecordedExchange { 0: req, ..Default::default() };
        assert_eq!(HttpMethodStub::from(&mut exchange), HttpMethodStub::from("OPTIONS"))
    }

    #[test]
    fn should_map_connect() {
        let req = RecordedRequest(Request::connect("http://localhost"));
        let mut exchange = RecordedExchange { 0: req, ..Default::default() };
        assert_eq!(HttpMethodStub::from(&mut exchange), HttpMethodStub::from("CONNECT"))
    }

    #[test]
    fn should_map_trace() {
        let req = RecordedRequest(Request::trace("http://localhost"));
        let mut exchange = RecordedExchange { 0: req, ..Default::default() };
        assert_eq!(HttpMethodStub::from(&mut exchange), HttpMethodStub::from("TRACE"))
    }
}