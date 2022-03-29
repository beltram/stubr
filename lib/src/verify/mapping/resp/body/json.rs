use async_std::task::block_on;
use serde_json::Value;

use crate::model::response::template::utils::TemplateExt;

use super::{
    json_templating::JsonBodyTemplatingVerifier,
    super::{
        StdResponse,
        super::{req::StdRequest, super::super::model::response::ResponseStub},
        Verifier,
    },
};

pub struct JsonBodyVerifier;

impl Verifier<'_> for JsonBodyVerifier {
    fn verify(self, stub: &'_ ResponseStub, name: &'_ str, req: &'_ mut StdRequest, resp: &'_ mut StdResponse) {
        if let Some(expected) = stub.body.json_body.to_owned() {
            let actual = block_on(async move { resp.0.body_json::<Value>().await.ok() });
            assert!(actual.is_some(), "\nVerification failed for stub '{}'. Expected json response body to be '{}' but none present", name, expected);
            let actual = actual.unwrap();
            if expected.has_template_expressions() {
                if stub.requires_response_templating() {
                    JsonBodyTemplatingVerifier { actual, expected }.verify(stub, name, req, &mut StdResponse::default());
                } else {
                    panic!("\nVerification failed for stub '{}'. No response template transformer present but template elements present in expected response json body '{}'", name, expected)
                }
            } else {
                assert_eq!(actual, expected, "\nVerification failed for stub '{}'. Expected json response body to be '{}' but was '{}'", name, expected, actual);
            }
        }
    }
}

#[cfg(test)]
mod json_body_verify_tests {
    use http_types::{Request, Response};
    use serde_json::json;

    use crate::model::response::body::BodyStub;

    use super::*;

    #[test]
    fn should_verify_json_body() {
        let body = json!({"name": "doe", "age": 42, "alive": true});
        let stub = ResponseStub { body: BodyStub { json_body: Some(body.clone()), ..Default::default() }, ..Default::default() };
        let mut req = StdRequest(Request::get("http://localhost/"));
        let mut resp = Response::new(200);
        resp.set_body(body);
        JsonBodyVerifier.verify(&stub, "json", &mut req, &mut StdResponse(resp));
    }

    #[should_panic(expected = "Verification failed for stub 'json'. Expected json response body to be '{\"name\":\"alice\"}' but was '{\"name\":\"bob\"}'")]
    #[test]
    fn verify_should_fail_when_wrong_json_body_returned() {
        let body = json!({"name": "alice"});
        let stub = ResponseStub { body: BodyStub { json_body: Some(body), ..Default::default() }, ..Default::default() };
        let mut req = StdRequest(Request::get("http://localhost/"));
        let mut resp = Response::new(200);
        resp.set_body(json!({"name": "bob"}));
        JsonBodyVerifier.verify(&stub, "json", &mut req, &mut StdResponse(resp));
    }

    #[should_panic(expected = "Verification failed for stub 'json'. Expected json response body to be '{\"name\":\"alice\"}' but none present")]
    #[test]
    fn verify_should_fail_when_json_body_expected_and_none_present() {
        let stub = ResponseStub { body: BodyStub { json_body: Some(json!({"name": "alice"})), ..Default::default() }, ..Default::default() };
        let mut req = StdRequest(Request::get("http://localhost/"));
        let mut resp = StdResponse(Response::new(200));
        JsonBodyVerifier.verify(&stub, "json", &mut req, &mut resp);
    }

    #[test]
    fn should_verify_when_response_templating_requested_but_no_template_element_present() {
        let body = json!({"name": "doe"});
        let stub = ResponseStub {
            body: BodyStub { json_body: Some(body.clone()), ..Default::default() },
            transformers: vec![String::from("response-template")],
            ..Default::default()
        };
        let mut req = StdRequest(Request::get("http://localhost/"));
        let mut resp = Response::new(200);
        resp.set_body(body);
        JsonBodyVerifier.verify(&stub, "json", &mut req, &mut StdResponse(resp));
    }

    #[should_panic(expected = "Verification failed for stub 'json'. No response template transformer present but template elements present in expected response json body '{\"name\":\"{{jsonPath request.body '$.name'}}\"}'")]
    #[test]
    fn verify_should_fail_when_response_templating_not_requested_but_template_element_present() {
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
        JsonBodyVerifier.verify(&stub, "json", &mut StdRequest(req), &mut StdResponse(resp));
    }
}
