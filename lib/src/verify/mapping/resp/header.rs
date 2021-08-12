use crate::model::response::ResponseStub;

use super::{StdResponse, super::req::StdRequest, Verifier};

pub struct HeaderVerifier;

impl Verifier<'_> for HeaderVerifier {
    fn verify(stub: &'_ ResponseStub, name: &'_ str, _req: &'_ StdRequest, resp: &'_ mut StdResponse) {
        if let Some(expected) = stub.headers.headers.as_ref() {
            for (expected_key, expected_value) in expected {
                if let Some(actual_value) = resp.0.header(expected_key.as_str()).and_then(|it| it.get(0)) {
                    let expected_value = expected_value.as_str().unwrap();
                    let actual_value = actual_value.as_str();
                    assert_eq!(actual_value, expected_value,
                               "Verification failed for stub '{}'. Expected response header '{}' to have value '{}' but was '{}'",
                               name, expected_key, expected_value, actual_value)
                } else {
                    panic!("Verification failed for stub '{}'. Expected one response header with key '{}' but none found", name, expected_key)
                }
            }
        }
    }
}

#[cfg(test)]
mod header_verify_tests {
    use std::iter::FromIterator;

    use http_types::{Request, Response};
    use serde_json::{Map, Value};

    use crate::model::response::{headers::HttpRespHeadersStub, ResponseStub};

    use super::*;

    #[test]
    fn should_verify_one() {
        let stub = ResponseStub {
            status: Some(200),
            headers: HttpRespHeadersStub {
                headers: Some(Map::from_iter(vec![(String::from("x-a"), Value::String(String::from("b")))]))
            },
            ..Default::default()
        };
        let req = StdRequest(Request::get("http://localhost/"));
        let mut resp = Response::new(200);
        resp.append_header("x-a", "b");
        let mut resp = StdResponse(resp);
        HeaderVerifier::verify(&stub, "one-header", &req, &mut resp);
    }

    #[test]
    fn should_verify_many() {
        let stub = ResponseStub {
            status: Some(200),
            headers: HttpRespHeadersStub {
                headers: Some(Map::from_iter(vec![
                    (String::from("x-a"), Value::String(String::from("b"))),
                    (String::from("x-c"), Value::String(String::from("d"))),
                ]))
            },
            ..Default::default()
        };
        let req = StdRequest(Request::get("http://localhost/"));
        let mut resp = Response::new(200);
        resp.append_header("x-a", "b");
        resp.append_header("x-c", "d");
        let mut resp = StdResponse(resp);
        HeaderVerifier::verify(&stub, "many-header", &req, &mut resp);
    }

    #[should_panic(expected = "Verification failed for stub 'missing-key'. Expected one response header with key 'x-a' but none found")]
    #[test]
    fn should_fail_when_missing() {
        let stub = ResponseStub {
            status: Some(200),
            headers: HttpRespHeadersStub {
                headers: Some(Map::from_iter(vec![(String::from("x-a"), Value::String(String::from("b")))]))
            },
            ..Default::default()
        };
        let req = StdRequest(Request::get("http://localhost/"));
        let mut resp = StdResponse(Response::new(200));
        HeaderVerifier::verify(&stub, "missing-key", &req, &mut resp);
    }

    #[should_panic(expected = "Verification failed for stub 'wrong-value'. Expected response header 'x-a' to have value 'b' but was 'c'")]
    #[test]
    fn should_fail_when_wrong_value() {
        let stub = ResponseStub {
            status: Some(200),
            headers: HttpRespHeadersStub {
                headers: Some(Map::from_iter(vec![(String::from("x-a"), Value::String(String::from("b")))]))
            },
            ..Default::default()
        };
        let req = StdRequest(Request::get("http://localhost/"));
        let mut resp = Response::new(200);
        resp.append_header("x-a", "c");
        let mut resp = StdResponse(resp);
        HeaderVerifier::verify(&stub, "wrong-value", &req, &mut resp);
    }
}