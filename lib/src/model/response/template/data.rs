use crate::wiremock::Request as WiremockRequest;
use http_types::Method;
use serde_json::Value;

use super::req_ext::{Headers, Queries, RequestExt};

#[derive(serde::Serialize, Debug)]
pub struct HandlebarsData<'a> {
    pub request: &'a RequestData<'a>,
    pub response: Option<&'a [u8]>,
    pub stub_name: Option<&'a str>,
    pub is_verify: bool,
}

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RequestData<'a> {
    path: &'a str,
    path_segments: Option<Vec<&'a str>>,
    url: &'a str,
    port: Option<u16>,
    method: Method,
    body: Option<Value>,
    query: Option<Queries<'a>>,
    headers: Option<Headers<'a>>,
}

impl Default for RequestData<'_> {
    fn default() -> Self {
        Self {
            path: "",
            path_segments: None,
            url: "",
            port: None,
            method: Method::Get,
            body: None,
            query: None,
            headers: None,
        }
    }
}

#[cfg(feature = "grpc")]
impl<'a> RequestData<'a> {
    pub fn from_grpc_request(req: &'a WiremockRequest, md: &protobuf::reflect::MessageDescriptor) -> Self {
        let body = crate::model::grpc::request::proto_to_json_str(req.body.as_slice(), md);
        let body = serde_json::from_str(&body).unwrap();
        Self {
            path: crate::model::grpc::request::path::GrpcPathMatcher::parse_svc_name(req),
            path_segments: None,
            url: "",
            port: None,
            method: Method::Post,
            body: Some(body),
            query: None,
            headers: None,
        }
    }
}

impl<'a> From<&'a WiremockRequest> for RequestData<'a> {
    fn from(req: &'a WiremockRequest) -> Self {
        Self {
            path: req.url.path(),
            path_segments: req.path_segments(),
            url: req.uri(),
            port: req.url.port(),
            method: req.method,
            body: req.body(),
            query: req.queries(),
            headers: req.headers(),
        }
    }
}

impl<'a> From<&'a mut http_types::Request> for RequestData<'a> {
    fn from(req: &'a mut http_types::Request) -> Self {
        let body = req.body_mut();
        Self {
            path: req.path(),
            path_segments: req.path_segments(),
            url: req.uri(),
            port: req.url().port(),
            method: req.method(),
            body,
            query: req.queries(),
            headers: req.headers(),
        }
    }
}

#[cfg(test)]
mod request_data_tests {
    use std::{borrow::Cow, collections::HashMap, str::FromStr};

    use http_types::{
        headers::{HeaderName, HeaderValue, HeaderValues},
        Method, Url,
    };
    use itertools::Itertools;
    use serde_json::{json, Value};

    use super::*;

    mod wiremock_data {
        use super::*;

        #[test]
        fn should_take_request_path() {
            let req = request("https://github.com/beltram/stubr", None, &[], None);
            assert_eq!(RequestData::from(&req).path, "/beltram/stubr");
        }

        #[test]
        fn should_take_request_path_ignoring_queries() {
            let req = request("https://github.com/beltram/stubr?branch=main&actor=beltram", None, &[], None);
            assert_eq!(RequestData::from(&req).path, "/beltram/stubr");
        }

        #[test]
        fn should_take_request_url() {
            let req = request("https://github.com/beltram/stubr?branch=main&actor=beltram", None, &[], None);
            assert_eq!(RequestData::from(&req).url, "/beltram/stubr?branch=main&actor=beltram");
        }

        #[test]
        fn should_take_request_url_when_no_query() {
            let req = request("https://github.com/beltram/stubr", None, &[], None);
            assert_eq!(RequestData::from(&req).url, "/beltram/stubr");
        }

        #[test]
        fn should_take_request_port() {
            let req = request("https://localhost:8080", None, &[], None);
            assert_eq!(RequestData::from(&req).port, Some(8080));
            let req = request("https://localhost:8081/api/pets", None, &[], None);
            assert_eq!(RequestData::from(&req).port, Some(8081));
        }

        #[test]
        fn should_not_take_request_port_when_absent() {
            let req = request("https://localhost", None, &[], None);
            assert!(RequestData::from(&req).port.is_none());
        }

        #[test]
        fn should_take_request_method() {
            let req = request("https://localhost", Some(Method::Get), &[], None);
            assert_eq!(RequestData::from(&req).method, Method::Get);
            let req = request("https://localhost", Some(Method::Post), &[], None);
            assert_eq!(RequestData::from(&req).method, Method::Post);
        }

        #[test]
        fn should_take_request_path_segments() {
            let req = request("https://localhost/one/two/three", None, &[], None);
            assert_eq!(RequestData::from(&req).path_segments, Some(vec!["one", "two", "three"]));
        }

        #[test]
        fn path_segments_should_be_empty_when_no_path() {
            let req = request("https://localhost", None, &[], None);
            assert!(RequestData::from(&req).path_segments.is_none());
        }

        #[test]
        fn should_take_request_text_body() {
            let req = request("https://localhost", Some(Method::Post), &[], Some("Lorem ipsum"));
            assert_eq!(
                RequestData::from(&req).body.as_ref().and_then(|it| it.as_str()),
                Some("Lorem ipsum")
            );
        }

        #[test]
        fn should_not_take_request_body_when_absent() {
            let req = request("https://localhost", None, &[], None);
            assert!(RequestData::from(&req).body.is_none());
        }

        #[test]
        fn should_take_request_json_body() {
            let req_body_str = "{\"name\": \"bob\", \"age\": 42}";
            let req = request("https://localhost", Some(Method::Post), &[], Some(req_body_str));
            assert_eq!(RequestData::from(&req).body, Some(json!({"name": "bob", "age": 42})));
        }

        #[test]
        fn should_take_request_query_parameters() {
            let req = request("https://localhost?age=1&weight=2", None, &[], None);
            let age = (Cow::Borrowed("age"), Value::from("1"));
            let weight = (Cow::Borrowed("weight"), Value::from("2"));
            assert_eq!(RequestData::from(&req).query, Some(HashMap::from_iter(vec![age, weight])));
        }

        #[test]
        fn should_take_request_multi_query_parameters() {
            let req = request("https://localhost?age=1&age=2", None, &[], None);
            let age = (Cow::Borrowed("age"), Value::from(vec!["1", "2"]));
            assert_eq!(RequestData::from(&req).query, Some(HashMap::from_iter(vec![age])));
        }

        #[test]
        fn request_query_parameters_should_be_none_when_missing() {
            let req = request("https://localhost", None, &[], None);
            assert!(RequestData::from(&req).query.is_none());
        }

        #[test]
        fn should_take_request_header_parameters() {
            let req = request("https://localhost", None, &[("x-1", &["1"]), ("x-2", &["2"])], None);
            let x1 = ("x-1", Value::from("1"));
            let x2 = ("x-2", Value::from("2"));
            assert_eq!(RequestData::from(&req).headers, Some(HashMap::from_iter(vec![x1, x2])));
        }

        #[test]
        fn should_take_multi_request_header_parameters() {
            let req = request("https://localhost", None, &[("x-multi", &["1", "2", "3"])], None);
            let header = ("x-multi", Value::from_iter(vec!["1", "2", "3"]));
            assert_eq!(RequestData::from(&req).headers, Some(HashMap::from_iter(vec![header])));
        }

        #[test]
        fn request_header_parameters_should_be_none_when_missing() {
            let req = request("https://localhost", None, &[], None);
            assert!(RequestData::from(&req).headers.is_none());
        }

        fn request(url: &str, method: Option<Method>, headers: &[(&str, &[&str])], body: Option<&str>) -> WiremockRequest {
            let url = Url::from_str(url).unwrap();
            let method = method.unwrap_or(Method::Get);
            let headers: HashMap<HeaderName, HeaderValues> = headers
                .iter()
                .filter_map(|(k, v)| {
                    HeaderName::from_str(k).ok().zip(Some(HeaderValues::from_iter(
                        v.iter().filter_map(|it| HeaderValue::from_str(it).ok()).collect_vec(),
                    )))
                })
                .collect();
            let body = body.map(|it| it.as_bytes().to_vec()).unwrap_or_default();
            WiremockRequest {
                url,
                method,
                headers,
                body,
            }
        }
    }

    mod http_types_data {
        use super::*;

        #[test]
        fn should_take_request_path() {
            let mut req = request("https://github.com/beltram/stubr", None, &[], None);
            assert_eq!(RequestData::from(&mut req).path, "/beltram/stubr");
        }

        #[test]
        fn should_take_request_path_ignoring_queries() {
            let mut req = request("https://github.com/beltram/stubr?branch=main&actor=beltram", None, &[], None);
            assert_eq!(RequestData::from(&mut req).path, "/beltram/stubr");
        }

        #[test]
        fn should_take_request_url() {
            let mut req = request("https://github.com/beltram/stubr?branch=main&actor=beltram", None, &[], None);
            assert_eq!(RequestData::from(&mut req).url, "/beltram/stubr?branch=main&actor=beltram");
        }

        #[test]
        fn should_take_request_url_when_no_query() {
            let mut req = request("https://github.com/beltram/stubr", None, &[], None);
            assert_eq!(RequestData::from(&mut req).url, "/beltram/stubr");
        }

        #[test]
        fn should_take_request_port() {
            let mut req = request("https://localhost:8080", None, &[], None);
            assert_eq!(RequestData::from(&mut req).port, Some(8080));
            let mut req = request("https://localhost:8081/api/pets", None, &[], None);
            assert_eq!(RequestData::from(&mut req).port, Some(8081));
        }

        #[test]
        fn should_not_take_request_port_when_absent() {
            let mut req = request("https://localhost", None, &[], None);
            assert!(RequestData::from(&mut req).port.is_none());
        }

        #[test]
        fn should_take_request_method() {
            let mut req = request("https://localhost", Some(Method::Get), &[], None);
            assert_eq!(RequestData::from(&mut req).method, Method::Get);
            let mut req = request("https://localhost", Some(Method::Post), &[], None);
            assert_eq!(RequestData::from(&mut req).method, Method::Post);
        }

        #[test]
        fn should_take_request_path_segments() {
            let mut req = request("https://localhost/one/two/three", None, &[], None);
            assert_eq!(RequestData::from(&mut req).path_segments, Some(vec!["one", "two", "three"]));
        }

        #[test]
        fn path_segments_should_be_empty_when_no_path() {
            let mut req = request("https://localhost", None, &[], None);
            assert!(RequestData::from(&mut req).path_segments.is_none());
        }

        #[test]
        fn should_take_request_text_body() {
            let mut req = request("https://localhost", Some(Method::Post), &[], Some("Lorem ipsum"));
            assert_eq!(
                RequestData::from(&mut req).body.as_ref().and_then(|it| it.as_str()),
                Some("Lorem ipsum")
            );
        }

        #[test]
        fn should_not_take_request_body_when_absent() {
            let mut req = request("https://localhost", None, &[], None);
            assert!(RequestData::from(&mut req).body.is_none());
        }

        #[test]
        fn should_take_request_json_body() {
            let req_body_str = "{\"name\": \"bob\", \"age\": 42}";
            let mut req = request("https://localhost", Some(Method::Post), &[], Some(req_body_str));
            assert_eq!(RequestData::from(&mut req).body, Some(json!({"name": "bob", "age": 42})));
        }

        #[test]
        fn should_take_request_query_parameters() {
            let mut req = request("https://localhost?age=1&weight=2", None, &[], None);
            let age = (Cow::Borrowed("age"), Value::from("1"));
            let weight = (Cow::Borrowed("weight"), Value::from("2"));
            assert_eq!(RequestData::from(&mut req).query, Some(HashMap::from_iter(vec![age, weight])));
        }

        #[test]
        fn should_take_request_multi_query_parameters() {
            let mut req = request("https://localhost?age=1&age=2", None, &[], None);
            let age = (Cow::Borrowed("age"), Value::from(vec!["1", "2"]));
            assert_eq!(RequestData::from(&mut req).query, Some(HashMap::from_iter(vec![age])));
        }

        #[test]
        fn request_query_parameters_should_be_none_when_missing() {
            let mut req = request("https://localhost", None, &[], None);
            assert!(RequestData::from(&mut req).query.is_none());
        }

        #[test]
        fn should_take_request_header_parameters() {
            let mut req = request("https://localhost", None, &[("x-1", &["1"]), ("x-2", &["2"])], None);
            let headers = RequestData::from(&mut req).headers.unwrap();
            assert_eq!(headers.get("x-1").unwrap(), &Value::from("1"));
            assert_eq!(headers.get("x-2").unwrap(), &Value::from("2"));
        }

        #[test]
        fn should_take_multi_request_header_parameters() {
            let mut req = request("https://localhost", None, &[("x-multi", &["1", "2", "3"])], None);
            let header = Value::from_iter(vec!["1", "2", "3"]);
            assert_eq!(
                RequestData::from(&mut req)
                    .headers
                    .as_ref()
                    .and_then(|h| h.get("x-multi"))
                    .unwrap(),
                &header
            );
        }

        #[test]
        fn request_header_parameters_should_be_none_when_missing() {
            let mut req = request("https://localhost", None, &[], None);
            // has by default "content-type": "application/octet-stream"
            assert_eq!(RequestData::from(&mut req).headers.unwrap().len(), 1);
        }

        fn request(url: &str, method: Option<Method>, headers: &[(&str, &[&str])], body: Option<&str>) -> http_types::Request {
            let url = Url::from_str(url).unwrap();
            let method = method.unwrap_or(Method::Get);
            let mut req = http_types::Request::new(method, url);
            headers
                .iter()
                .filter_map(|(k, v)| {
                    HeaderName::from_str(k).ok().zip(Some(HeaderValues::from_iter(
                        v.iter().filter_map(|it| HeaderValue::from_str(it).ok()).collect_vec(),
                    )))
                })
                .for_each(|(k, v)| {
                    req.insert_header(k, &v);
                });
            let body = body.map(|it| it.as_bytes().to_vec()).unwrap_or_default();
            req.set_body(body);
            req
        }
    }
}
