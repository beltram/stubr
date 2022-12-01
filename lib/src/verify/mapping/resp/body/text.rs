use async_std::task::block_on;

use crate::model::response::{
    template::{data::RequestData, utils::TemplateExt},
    ResponseStub,
};

use super::{
    super::{StdResponse, Verifier},
    text_templating::TextBodyTemplatingVerifier,
};

pub struct TextBodyVerifier;

impl Verifier<'_> for TextBodyVerifier {
    fn verify(self, stub: &'_ ResponseStub, name: &'_ str, req: &'_ RequestData, resp: &'_ mut StdResponse) {
        if let Some(expected) = stub.body.body.to_owned() {
            let actual = block_on(async move { resp.0.body_string().await.ok() }).filter(|it| !it.is_empty());
            assert!(
                actual.is_some(),
                "\nVerification failed for stub '{}'. Expected response body to be '{}' but none present",
                name,
                expected
            );
            let actual = actual.unwrap();
            if expected.has_template_expressions() {
                if stub.requires_response_templating() {
                    TextBodyTemplatingVerifier { actual, expected }.verify(stub, name, req, &mut StdResponse::default());
                } else {
                    panic!("\nVerification failed for stub '{}'. No response template transformer present but template elements present in expected response text body '{}'", name, expected)
                }
            } else {
                assert_eq!(
                    actual, expected,
                    "\nVerification failed for stub '{}'. Expected response body to be '{}' but was '{}'",
                    name, expected, actual
                );
            }
        }
    }
}

#[cfg(test)]
mod text_body_verify_tests {
    use http_types::{Request, Response};

    use crate::model::response::body::BodyStub;

    use super::*;

    #[test]
    fn should_verify() {
        let stub = ResponseStub {
            body: BodyStub {
                body: Some("alice".to_string()),
                ..Default::default()
            },
            ..Default::default()
        };
        let mut req = Request::get("http://localhost/");
        let mut resp = Response::new(200);
        resp.set_body("alice".to_string());
        TextBodyVerifier.verify(&stub, "text", &RequestData::from(&mut req), &mut StdResponse(resp));
    }

    #[should_panic(expected = "Verification failed for stub 'text'. Expected response body to be 'alice' but was 'bob'")]
    #[test]
    fn verify_should_fail_when_wrong_text_body_returned() {
        let stub = ResponseStub {
            body: BodyStub {
                body: Some("alice".to_string()),
                ..Default::default()
            },
            ..Default::default()
        };
        let mut req = Request::get("http://localhost/");
        let mut resp = Response::new(200);
        resp.set_body("bob".to_string());
        TextBodyVerifier.verify(&stub, "text", &RequestData::from(&mut req), &mut StdResponse(resp));
    }

    #[should_panic(expected = "Verification failed for stub 'text'. Expected response body to be 'alice' but none present")]
    #[test]
    fn verify_should_fail_when_text_body_expected_and_none_present() {
        let stub = ResponseStub {
            body: BodyStub {
                body: Some("alice".to_string()),
                ..Default::default()
            },
            ..Default::default()
        };
        let mut req = Request::get("http://localhost/");
        TextBodyVerifier.verify(
            &stub,
            "text",
            &RequestData::from(&mut req),
            &mut StdResponse(Response::new(200)),
        );
    }

    #[test]
    fn should_verify_when_response_templating_requested_but_no_template_element_present() {
        let stub = ResponseStub {
            body: BodyStub {
                body: Some("alice".to_string()),
                ..Default::default()
            },
            transformers: vec![String::from("response-template")],
            ..Default::default()
        };
        let mut req = Request::get("http://localhost/");
        let mut resp = Response::new(200);
        resp.set_body("alice".to_string());
        TextBodyVerifier.verify(&stub, "text", &RequestData::from(&mut req), &mut StdResponse(resp));
    }

    #[should_panic(
        expected = "Verification failed for stub 'text'. No response template transformer present but template elements present in expected response text body '{{anyNonBlankString}}'"
    )]
    #[test]
    fn verify_should_fail_when_response_templating_not_requested_but_template_element_present() {
        let stub = ResponseStub {
            body: BodyStub {
                body: Some("{{anyNonBlankString}}".to_string()),
                ..Default::default()
            },
            ..Default::default()
        };
        let mut req = Request::get("http://localhost/");
        let mut resp = Response::new(200);
        resp.set_body("alice".to_string());
        TextBodyVerifier.verify(&stub, "text", &RequestData::from(&mut req), &mut StdResponse(resp));
    }
}
