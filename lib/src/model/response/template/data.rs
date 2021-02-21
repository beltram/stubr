use http_types::Method;
use serde::Serialize;
use wiremock::Request;

use super::req_ext::{Headers, Queries, RequestExt};

#[derive(Serialize)]
pub struct HandlebarsData<'a> { request: RequestData<'a> }

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RequestData<'a> {
    path: &'a str,
    path_segments: Option<Vec<&'a str>>,
    url: &'a str,
    port: Option<u16>,
    method: Method,
    body: Option<&'a str>,
    query: Option<Queries<'a>>,
    headers: Option<Headers<'a>>,
}

impl<'a> From<&'a Request> for HandlebarsData<'a> {
    fn from(req: &'a Request) -> Self {
        Self { request: req.into() }
    }
}

impl<'a> From<&'a Request> for RequestData<'a> {
    fn from(req: &'a Request) -> Self {
        Self {
            path: req.url.path(),
            path_segments: req.path_segments(),
            url: req.url(),
            port: req.url.port(),
            method: req.method,
            body: req.body(),
            query: req.queries(),
            headers: req.headers(),
        }
    }
}

#[cfg(test)]
mod request_data_tests {
    use std::{borrow::Cow, collections::HashMap, iter::FromIterator, str::FromStr};

    use http_types::{headers::{HeaderName, HeaderValue, HeaderValues}, Method, Url};
    use serde_json::Value;

    use super::*;
    use itertools::Itertools;

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
    fn should_take_request_body() {
        let req = request("https://localhost", Some(Method::Post), &[], Some("Lorem ipsum"));
        assert_eq!(RequestData::from(&req).body, Some("Lorem ipsum"));
        let req = request("https://localhost", None, &[], None);
        assert!(RequestData::from(&req).body.is_none());
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

    fn request(url: &str, method: Option<Method>, headers: &[(&str, &[&str])], body: Option<&str>) -> Request {
        let url = Url::from_str(url).unwrap();
        let method = method.unwrap_or(Method::Get);
        let headers: HashMap<HeaderName, HeaderValues> = headers.iter()
            .filter_map(|(k, v)| {
                HeaderName::from_str(k).ok()
                    .zip(Some(HeaderValues::from_iter(v.iter().filter_map(|it| HeaderValue::from_str(it).ok()).collect_vec())))
            })
            .collect();
        let body = body.map(|it| it.as_bytes().to_vec()).unwrap_or_default();
        Request { url, method, headers, body }
    }
}