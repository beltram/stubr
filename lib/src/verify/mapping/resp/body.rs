use async_std::task::block_on;
use serde_json::Value;

use crate::model::response::ResponseStub;

use super::{StdResponse, super::req::StdRequest, Verifier};

pub struct BodyVerifier;

impl BodyVerifier {}

impl Verifier<'_> for BodyVerifier {
    fn verify(stub: &'_ ResponseStub, name: &'_ str, _req: &'_ StdRequest, resp: &'_ mut StdResponse) {
        if let Some(expected) = stub.body.json_body.as_ref() {
            let actual = block_on(async move { resp.0.body_json::<Value>().await.ok() });
            assert!(actual.is_some(), "Verification failed for stub '{}'. Expected json response body to be '{}' but none present", name, expected);
            let actual = actual.as_ref().unwrap();
            assert_eq!(actual, expected, "Verification failed for stub '{}'. Expected json response body to be '{}' but was '{}'", name, expected, actual);
        } else if let Some(expected) = stub.body.body.as_ref() {
            let actual = block_on(async move { resp.0.body_string().await.ok() }).filter(|it| !it.is_empty());
            assert!(actual.is_some(), "Verification failed for stub '{}'. Expected text response body to be '{}' but none present", name, expected);
            let actual = actual.as_ref().unwrap();
            assert_eq!(actual, expected, "Verification failed for stub '{}'. Expected text response body to be '{}' but was '{}'", name, expected, actual);
        }
    }
}

#[cfg(test)]
mod body_verify_tests {
    use http_types::{Request, Response};
    use serde_json::json;

    use crate::model::response::body::BodyStub;

    use super::*;

    mod json {
        use super::*;

        #[test]
        fn should_verify_json_body() {
            let body = json!({"name": "doe"});
            let stub = ResponseStub { body: BodyStub { json_body: Some(body.clone()), ..Default::default() }, ..Default::default() };
            let req = StdRequest(Request::get("http://localhost/"));
            let mut resp = Response::new(200);
            resp.set_body(body);
            let mut resp = StdResponse(resp);
            BodyVerifier::verify(&stub, "json", &req, &mut resp);
        }

        #[should_panic(expected = "Verification failed for stub 'json'. Expected json response body to be '{\"name\":\"alice\"}' but was '{\"name\":\"bob\"}'")]
        #[test]
        fn verify_should_fail_when_wrong_json_body_returned() {
            let body = json!({"name": "alice"});
            let stub = ResponseStub { body: BodyStub { json_body: Some(body), ..Default::default() }, ..Default::default() };
            let req = StdRequest(Request::get("http://localhost/"));
            let mut resp = Response::new(200);
            resp.set_body(json!({"name": "bob"}));
            let mut resp = StdResponse(resp);
            BodyVerifier::verify(&stub, "json", &req, &mut resp);
        }

        #[should_panic(expected = "Verification failed for stub 'json'. Expected json response body to be '{\"name\":\"alice\"}' but none present")]
        #[test]
        fn verify_should_fail_when_json_body_expected_and_none_present() {
            let stub = ResponseStub { body: BodyStub { json_body: Some(json!({"name": "alice"})), ..Default::default() }, ..Default::default() };
            let req = StdRequest(Request::get("http://localhost/"));
            let mut resp = StdResponse(Response::new(200));
            BodyVerifier::verify(&stub, "json", &req, &mut resp);
        }

        #[test]
        fn verify_should_not_fail_when_no_json_body_expected_and_one_present() {
            let stub = ResponseStub::default();
            let req = StdRequest(Request::get("http://localhost/"));
            let mut resp = Response::new(200);
            resp.set_body(json!({"name": "bob"}));
            let mut resp = StdResponse(resp);
            BodyVerifier::verify(&stub, "json", &req, &mut resp);
        }
    }

    mod text {
        use super::*;

        #[test]
        fn should_verify_text_body() {
            let stub = ResponseStub { body: BodyStub { body: Some("alice".to_string()), ..Default::default() }, ..Default::default() };
            let req = StdRequest(Request::get("http://localhost/"));
            let mut resp = Response::new(200);
            resp.set_body("alice".to_string());
            let mut resp = StdResponse(resp);
            BodyVerifier::verify(&stub, "text", &req, &mut resp);
        }

        #[should_panic(expected = "Verification failed for stub 'text'. Expected text response body to be 'alice' but was 'bob'")]
        #[test]
        fn verify_should_fail_when_wrong_text_body_returned() {
            let stub = ResponseStub { body: BodyStub { body: Some("alice".to_string()), ..Default::default() }, ..Default::default() };
            let req = StdRequest(Request::get("http://localhost/"));
            let mut resp = Response::new(200);
            resp.set_body("bob".to_string());
            let mut resp = StdResponse(resp);
            BodyVerifier::verify(&stub, "text", &req, &mut resp);
        }

        #[should_panic(expected = "Verification failed for stub 'text'. Expected text response body to be 'alice' but none present")]
        #[test]
        fn verify_should_fail_when_text_body_expected_and_none_present() {
            let stub = ResponseStub { body: BodyStub { body: Some("alice".to_string()), ..Default::default() }, ..Default::default() };
            let req = StdRequest(Request::get("http://localhost/"));
            let mut resp = StdResponse(Response::new(200));
            BodyVerifier::verify(&stub, "text", &req, &mut resp);
        }

        #[test]
        fn verify_should_fail_when_no_text_body_expected_and_one_present() {
            let stub = ResponseStub::default();
            let req = StdRequest(Request::get("http://localhost/"));
            let mut resp = Response::new(200);
            resp.set_body("bob".to_string());
            let mut resp = StdResponse(resp);
            BodyVerifier::verify(&stub, "text", &req, &mut resp);
        }
    }
}
