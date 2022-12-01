use http_types::Method;

use crate::model::request::method::{HttpMethodStub, Verb};

impl From<&HttpMethodStub> for Method {
    fn from(method: &HttpMethodStub) -> Self {
        match method.0 {
            Verb::Get => Method::Get,
            Verb::Any | Verb::Post => Method::Post,
            Verb::Put => Method::Put,
            Verb::Delete => Method::Delete,
            Verb::Patch => Method::Patch,
            Verb::Head => Method::Head,
            Verb::Options => Method::Options,
            Verb::Connect => Method::Connect,
            Verb::Trace => Method::Trace,
        }
    }
}

#[cfg(test)]
mod method_mapping_tests {
    use crate::model::request::method::HttpMethodStub;

    use super::*;

    #[test]
    fn should_map_get() {
        assert_eq!(Method::from(&HttpMethodStub::from("GET")), Method::Get)
    }

    #[test]
    fn should_map_post() {
        assert_eq!(Method::from(&HttpMethodStub::from("POST")), Method::Post)
    }

    #[test]
    fn should_map_put() {
        assert_eq!(Method::from(&HttpMethodStub::from("PUT")), Method::Put)
    }

    #[test]
    fn should_map_delete() {
        assert_eq!(Method::from(&HttpMethodStub::from("DELETE")), Method::Delete)
    }

    #[test]
    fn should_map_patch() {
        assert_eq!(Method::from(&HttpMethodStub::from("PATCH")), Method::Patch)
    }

    #[test]
    fn should_map_head() {
        assert_eq!(Method::from(&HttpMethodStub::from("HEAD")), Method::Head)
    }

    #[test]
    fn should_map_options() {
        assert_eq!(Method::from(&HttpMethodStub::from("OPTIONS")), Method::Options)
    }

    #[test]
    fn should_map_connect() {
        assert_eq!(Method::from(&HttpMethodStub::from("CONNECT")), Method::Connect)
    }

    #[test]
    fn should_map_trace() {
        assert_eq!(Method::from(&HttpMethodStub::from("TRACE")), Method::Trace)
    }

    #[test]
    fn should_map_any_to_post() {
        assert_eq!(Method::from(&HttpMethodStub::from("ANY")), Method::Post)
    }
}
