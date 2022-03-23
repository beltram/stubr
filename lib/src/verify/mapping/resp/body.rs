use std::borrow::BorrowMut;

use async_std::task::block_on;
use serde_json::Value;

use super::{
    StdResponse,
    super::{
        req::StdRequest,
        super::super::model::response::{
            ResponseStub,
            template::{data::HandlebarsData, HandlebarTemplatable},
        },
    },
    Verifier,
};

pub struct BodyVerifier;

impl Verifier<'_> for BodyVerifier {
    fn verify(stub: &'_ ResponseStub, name: &'_ str, req: &'_ mut StdRequest, resp: &'_ mut StdResponse) {
        if let Some(expected) = stub.body.json_body.as_ref() {
            let actual = block_on(async move { resp.0.body_json::<Value>().await.ok() });
            assert!(actual.is_some(), "\nVerification failed for stub '{}'. Expected json response body to be '{}' but none present", name, expected);
            let actual = actual.as_ref().unwrap();
            if stub.requires_response_templating() {
                if let Some(obj) = expected.as_object() {
                    stub.body.register_json_body_template(obj.values());
                } else if let Some(arr) = expected.as_array() {
                    stub.body.register_json_body_template(arr.iter())
                }
                let expected = stub.body.render_json_body(Some(expected), &HandlebarsData::from(req.0.borrow_mut()))
                    .unwrap_or_else(|| panic!("Failed rendering response template for '{}'", name));
                assert_eq!(actual, &expected, "\nVerification failed for stub '{}'. Expected json response body to be '{}' but was '{}'", name, expected, actual);
            } else {
                assert_eq!(actual, expected, "\nVerification failed for stub '{}'. Expected json response body to be '{}' but was '{}'", name, expected, actual);
            }
        } else if let Some(expected) = stub.body.body.as_ref() {
            let actual = block_on(async move { resp.0.body_string().await.ok() }).filter(|it| !it.is_empty());
            assert!(actual.is_some(), "\nVerification failed for stub '{}'. Expected text response body to be '{}' but none present", name, expected);
            let actual = actual.as_ref().unwrap();
            if stub.requires_response_templating() {
                stub.body.register(expected, expected);
                let expected = stub.body.render(expected, &HandlebarsData::from(req.0.borrow_mut()));
                assert_eq!(actual, &expected, "\nVerification failed for stub '{}'. Expected text response body to be '{}' but was '{}'", name, expected, actual);
            } else {
                assert_eq!(actual, expected, "\nVerification failed for stub '{}'. Expected text response body to be '{}' but was '{}'", name, expected, actual);
            }
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
            let mut req = StdRequest(Request::get("http://localhost/"));
            let mut resp = Response::new(200);
            resp.set_body(body);
            BodyVerifier::verify(&stub, "json", &mut req, &mut StdResponse(resp));
        }

        #[should_panic(expected = "Verification failed for stub 'json'. Expected json response body to be '{\"name\":\"alice\"}' but was '{\"name\":\"bob\"}'")]
        #[test]
        fn verify_should_fail_when_wrong_json_body_returned() {
            let body = json!({"name": "alice"});
            let stub = ResponseStub { body: BodyStub { json_body: Some(body), ..Default::default() }, ..Default::default() };
            let mut req = StdRequest(Request::get("http://localhost/"));
            let mut resp = Response::new(200);
            resp.set_body(json!({"name": "bob"}));
            BodyVerifier::verify(&stub, "json", &mut req, &mut StdResponse(resp));
        }

        #[should_panic(expected = "Verification failed for stub 'json'. Expected json response body to be '{\"name\":\"alice\"}' but none present")]
        #[test]
        fn verify_should_fail_when_json_body_expected_and_none_present() {
            let stub = ResponseStub { body: BodyStub { json_body: Some(json!({"name": "alice"})), ..Default::default() }, ..Default::default() };
            let mut req = StdRequest(Request::get("http://localhost/"));
            let mut resp = StdResponse(Response::new(200));
            BodyVerifier::verify(&stub, "json", &mut req, &mut resp);
        }

        #[test]
        fn verify_should_not_fail_when_no_json_body_expected_and_one_present() {
            let stub = ResponseStub::default();
            let mut req = StdRequest(Request::get("http://localhost/"));
            let mut resp = Response::new(200);
            resp.set_body(json!({"name": "bob"}));
            BodyVerifier::verify(&stub, "json", &mut req, &mut StdResponse(resp));
        }
    }

    mod text {
        use super::*;

        #[test]
        fn should_verify_text_body() {
            let stub = ResponseStub { body: BodyStub { body: Some("alice".to_string()), ..Default::default() }, ..Default::default() };
            let mut req = StdRequest(Request::get("http://localhost/"));
            let mut resp = Response::new(200);
            resp.set_body("alice".to_string());
            BodyVerifier::verify(&stub, "text", &mut req, &mut StdResponse(resp));
        }

        #[should_panic(expected = "Verification failed for stub 'text'. Expected text response body to be 'alice' but was 'bob'")]
        #[test]
        fn verify_should_fail_when_wrong_text_body_returned() {
            let stub = ResponseStub { body: BodyStub { body: Some("alice".to_string()), ..Default::default() }, ..Default::default() };
            let mut req = StdRequest(Request::get("http://localhost/"));
            let mut resp = Response::new(200);
            resp.set_body("bob".to_string());
            BodyVerifier::verify(&stub, "text", &mut req, &mut StdResponse(resp));
        }

        #[should_panic(expected = "Verification failed for stub 'text'. Expected text response body to be 'alice' but none present")]
        #[test]
        fn verify_should_fail_when_text_body_expected_and_none_present() {
            let stub = ResponseStub { body: BodyStub { body: Some("alice".to_string()), ..Default::default() }, ..Default::default() };
            let mut req = StdRequest(Request::get("http://localhost/"));
            BodyVerifier::verify(&stub, "text", &mut req, &mut StdResponse(Response::new(200)));
        }

        #[test]
        fn verify_should_fail_when_no_text_body_expected_and_one_present() {
            let stub = ResponseStub::default();
            let mut req = StdRequest(Request::get("http://localhost/"));
            let mut resp = Response::new(200);
            resp.set_body("bob".to_string());
            BodyVerifier::verify(&stub, "text", &mut req, &mut StdResponse(resp));
        }
    }

    mod response_templating {
        use super::*;

        #[test]
        fn should_verify_json_body() {
            let body = json!({"name": "alice"});
            let body_template = json!({"name": "{{jsonPath request.body '$.name'}}"});
            let stub = ResponseStub {
                body: BodyStub { json_body: Some(body_template), ..Default::default() },
                transformers: vec![String::from("response-template")],
                ..Default::default()
            };
            let mut req = Request::post("http://localhost/");
            req.set_body(body.clone());
            let mut resp = Response::new(200);
            resp.set_body(body);
            BodyVerifier::verify(&stub, "json", &mut StdRequest(req), &mut StdResponse(resp));
        }

        #[test]
        fn should_verify_json_array_body() {
            let stub = ResponseStub {
                body: BodyStub { json_body: Some(json!(["{{jsonPath request.body '$.name'}}"])), ..Default::default() },
                transformers: vec![String::from("response-template")],
                ..Default::default()
            };
            let mut req = Request::post("http://localhost/");
            req.set_body(json!({"name": "alice"}));
            let mut resp = Response::new(200);
            resp.set_body(json!(["alice"]));
            BodyVerifier::verify(&stub, "json", &mut StdRequest(req), &mut StdResponse(resp));
        }

        #[should_panic(expected = "Verification failed for stub 'json'. Expected json response body to be '{\"name\":\"{{jsonPath request.body '$.name'}}\"}' but was '{\"name\":\"alice\"}'")]
        #[test]
        fn should_fail_verifying_json_when_no_transformer() {
            let body = json!({"name": "alice"});
            let body_template = json!({"name": "{{jsonPath request.body '$.name'}}"});
            let stub = ResponseStub {
                body: BodyStub { json_body: Some(body_template), ..Default::default() },
                ..Default::default()
            };
            let mut req = Request::post("http://localhost/");
            req.set_body(body.clone());
            let mut resp = Response::new(200);
            resp.set_body(body);
            BodyVerifier::verify(&stub, "json", &mut StdRequest(req), &mut StdResponse(resp));
        }

        #[test]
        fn should_verify_text_body() {
            let path = "/one/two/three";
            let stub = ResponseStub {
                body: BodyStub { body: Some("{{request.path}}".to_string()), ..Default::default() },
                transformers: vec![String::from("response-template")],
                ..Default::default()
            };
            let req = Request::get(format!("http://localhost{}", path).as_str());
            let mut resp = Response::new(200);
            resp.set_body(path);
            BodyVerifier::verify(&stub, "text", &mut StdRequest(req), &mut StdResponse(resp));
        }

        #[should_panic(expected = "Verification failed for stub 'text'. Expected text response body to be '{{request.path}}' but was '/one/two/three'")]
        #[test]
        fn should_fail_verifying_text_body_when_no_transformer() {
            let path = "/one/two/three";
            let stub = ResponseStub {
                body: BodyStub { body: Some("{{request.path}}".to_string()), ..Default::default() },
                ..Default::default()
            };
            let req = Request::get(format!("http://localhost{}", path).as_str());
            let mut resp = Response::new(200);
            resp.set_body(path);
            BodyVerifier::verify(&stub, "text", &mut StdRequest(req), &mut StdResponse(resp));
        }

        #[test]
        fn should_verify_json_body_from_path_segments() {
            let id = 1;
            let stub = ResponseStub {
                body: BodyStub { json_body: Some(json!({"id": "{{request.pathSegments.[0]}}"})), ..Default::default() },
                transformers: vec![String::from("response-template")],
                ..Default::default()
            };
            let req = Request::get(format!("http://localhost/{}", id).as_str());
            let mut resp = Response::new(200);
            resp.set_body(json!({"id": id}));
            BodyVerifier::verify(&stub, "json", &mut StdRequest(req), &mut StdResponse(resp));
        }
    }
}
