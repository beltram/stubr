use std::borrow::BorrowMut;

use super::super::{
    StdResponse,
    super::{
        req::StdRequest,
        super::super::model::response::{
            ResponseStub,
            template::{
                data::{HandlebarsData, RequestData},
                HandlebarTemplatable,
                verify::Predictable,
            },
        },
    },
    Verifier,
};

pub struct TextBodyTemplatingVerifier {
    pub actual: String,
    pub expected: String,
}

impl Verifier<'_> for TextBodyTemplatingVerifier {
    fn verify(self, stub: &'_ ResponseStub, name: &'_ str, req: &'_ mut StdRequest, _: &'_ mut StdResponse) {
        let data = HandlebarsData {
            request: RequestData::from(req.0.borrow_mut()),
            response: Some(self.actual.as_bytes()),
            is_verify: true,
            stub_name: Some(name),
        };
        stub.body.register(&self.expected, &self.expected);
        let expected = stub.body.render(&self.expected, &data);
        if self.expected.is_predictable() {
            assert_eq!(self.actual, expected, "\nVerification failed for stub '{}'. Expected text response body to be '{}' but was '{}'", name, expected, self.actual);
        }
    }
}

#[cfg(test)]
mod text_body_templating_verify_tests {
    use http_types::{Request, Response};

    use crate::model::response::body::BodyStub;

    use super::*;

    mod from_req {
        use super::*;

        #[test]
        fn should_verify_text_body() {
            let actual = "/one/two/three".to_string();
            let expected = "{{request.path}}".to_string();
            let stub = ResponseStub {
                body: BodyStub { body: Some(expected.clone()), ..Default::default() },
                transformers: vec![String::from("response-template")],
                ..Default::default()
            };
            let req = Request::get(format!("http://localhost{}", &actual).as_str());
            let mut resp = Response::new(200);
            resp.set_body(actual.as_str());
            TextBodyTemplatingVerifier { actual, expected }
                .verify(&stub, "text", &mut StdRequest(req), &mut StdResponse(resp));
        }

        #[test]
        fn should_verify_text_body_when_many_templates() {
            let actual = "one-two-three".to_string();
            let expected = "{{request.pathSegments.[0]}}-{{request.pathSegments.[1]}}-{{request.pathSegments.[2]}}".to_string();
            let stub = ResponseStub {
                body: BodyStub { body: Some(expected.clone()), ..Default::default() },
                transformers: vec![String::from("response-template")],
                ..Default::default()
            };
            let req = Request::get("http://localhost/one/two/three");
            let mut resp = Response::new(200);
            resp.set_body(actual.as_str());
            TextBodyTemplatingVerifier { actual, expected }
                .verify(&stub, "text", &mut StdRequest(req), &mut StdResponse(resp));
        }

        #[should_panic(expected = "Verification failed for stub 'text'. Expected text response body to be 'three-two-one' but was 'one-two-three'")]
        #[test]
        fn verify_text_body_should_fail_when_not_eq() {
            let actual = "one-two-three".to_string();
            let expected = "{{request.pathSegments.[2]}}-{{request.pathSegments.[1]}}-{{request.pathSegments.[0]}}".to_string();
            let stub = ResponseStub {
                body: BodyStub { body: Some(expected.clone()), ..Default::default() },
                transformers: vec![String::from("response-template")],
                ..Default::default()
            };
            let req = Request::get("http://localhost/one/two/three");
            let mut resp = Response::new(200);
            resp.set_body(actual.as_str());
            TextBodyTemplatingVerifier { actual, expected }
                .verify(&stub, "text", &mut StdRequest(req), &mut StdResponse(resp));
        }
    }

    mod any {
        use super::*;

        #[should_panic(expected = "Cannot verify stub 'text' because response body '{{anyNonBlankString}}-{{anyNonEmptyString}}' is not verifiable")]
        #[test]
        fn verify_text_body_when_many_rnd_template_should_not_verify_because_not_predictable() {
            let actual = "anything".to_string();
            let expected = "{{anyNonBlankString}}-{{anyNonEmptyString}}".to_string();
            let stub = ResponseStub {
                body: BodyStub { body: Some(expected.clone()), ..Default::default() },
                transformers: vec![String::from("response-template")],
                ..Default::default()
            };
            let req = Request::get("http://localhost");
            let mut resp = Response::new(200);
            resp.set_body(actual.as_str());
            TextBodyTemplatingVerifier { actual, expected }
                .verify(&stub, "text", &mut StdRequest(req), &mut StdResponse(resp));
        }
    }

    mod any_regex {
        use super::*;

        #[test]
        fn should_verify_text_body() {
            let actual = "FR".to_string();
            let expected = "{{anyRegex '[A-Z]{2}'}}".to_string();
            let stub = ResponseStub {
                body: BodyStub { body: Some(expected.clone()), ..Default::default() },
                transformers: vec![String::from("response-template")],
                ..Default::default()
            };
            let req = Request::get("http://localhost");
            let mut resp = Response::new(200);
            resp.set_body(actual.as_str());
            TextBodyTemplatingVerifier { actual, expected }
                .verify(&stub, "regex", &mut StdRequest(req), &mut StdResponse(resp));
        }

        #[should_panic(expected = "Verification failed for stub 'regex'. Expected response body to match '[A-Z]{2}' but was 'fr'")]
        #[test]
        fn verify_text_body_should_fail_when_regex_does_not_match() {
            let actual = "fr".to_string();
            let expected = "{{anyRegex '[A-Z]{2}'}}".to_string();
            let stub = ResponseStub {
                body: BodyStub { body: Some(expected.clone()), ..Default::default() },
                transformers: vec![String::from("response-template")],
                ..Default::default()
            };
            let req = Request::get("http://localhost");
            let mut resp = Response::new(200);
            resp.set_body(actual.as_str());
            TextBodyTemplatingVerifier { actual, expected }
                .verify(&stub, "regex", &mut StdRequest(req), &mut StdResponse(resp));
        }

        #[should_panic(expected = "Verification failed for stub 'regex'. Expected text response body to match '[A-Z]{2}' but no response body was present")]
        #[test]
        fn verify_text_body_should_fail_when_body_absent() {
            let expected = "{{anyRegex '[A-Z]{2}'}}".to_string();
            let stub = ResponseStub {
                body: BodyStub { body: Some(expected.clone()), ..Default::default() },
                transformers: vec![String::from("response-template")],
                ..Default::default()
            };
            let req = Request::get("http://localhost");
            TextBodyTemplatingVerifier { actual: String::new(), expected }
                .verify(&stub, "regex", &mut StdRequest(req), &mut StdResponse(Response::new(200)));
        }
    }

    mod any_non_blank_string {
        use super::*;

        #[test]
        fn should_verify_text_body() {
            let actual = "azerty".to_string();
            let expected = "{{anyNonBlankString}}".to_string();
            let stub = ResponseStub {
                body: BodyStub { body: Some(expected.clone()), ..Default::default() },
                transformers: vec![String::from("response-template")],
                ..Default::default()
            };
            let req = Request::get("http://localhost");
            let mut resp = Response::new(200);
            resp.set_body(actual.as_str());
            TextBodyTemplatingVerifier { actual, expected }
                .verify(&stub, "blank", &mut StdRequest(req), &mut StdResponse(resp));
        }

        #[should_panic(expected = "Verification failed for stub 'blank'. Expected response body to match '{{anyNonBlankString}}' but response body was ' '")]
        #[test]
        fn verify_text_body_should_fail_when_body_contains_space() {
            let actual = " ".to_string();
            let expected = "{{anyNonBlankString}}".to_string();
            let stub = ResponseStub {
                body: BodyStub { body: Some(expected.clone()), ..Default::default() },
                transformers: vec![String::from("response-template")],
                ..Default::default()
            };
            let req = Request::get("http://localhost");
            let mut resp = Response::new(200);
            resp.set_body(actual.as_str());
            TextBodyTemplatingVerifier { actual, expected }
                .verify(&stub, "blank", &mut StdRequest(req), &mut StdResponse(resp));
        }

        #[should_panic(expected = "Verification failed for stub 'blank'. Expected text response body to match '{{anyNonBlankString}}' but no response body was present")]
        #[test]
        fn verify_text_body_should_fail_when_body_absent() {
            let expected = "{{anyNonBlankString}}".to_string();
            let stub = ResponseStub {
                body: BodyStub { body: Some(expected.clone()), ..Default::default() },
                transformers: vec![String::from("response-template")],
                ..Default::default()
            };
            let req = Request::get("http://localhost");
            TextBodyTemplatingVerifier { actual: String::new(), expected }
                .verify(&stub, "blank", &mut StdRequest(req), &mut StdResponse(Response::new(200)));
        }
    }

    mod any_non_empty_string {
        use super::*;

        #[test]
        fn should_verify_text_body() {
            let actual = "azerty".to_string();
            let expected = "{{anyNonEmptyString}}".to_string();
            let stub = ResponseStub {
                body: BodyStub { body: Some(expected.clone()), ..Default::default() },
                transformers: vec![String::from("response-template")],
                ..Default::default()
            };
            let req = Request::get("http://localhost");
            let mut resp = Response::new(200);
            resp.set_body(actual.as_str());
            TextBodyTemplatingVerifier { actual, expected }
                .verify(&stub, "blank", &mut StdRequest(req), &mut StdResponse(resp));
        }

        #[should_panic(expected = "Verification failed for stub 'blank'. Expected text response body to match '{{anyNonEmptyString}}' but no response body was present")]
        #[test]
        fn verify_text_body_should_fail_when_body_empty() {
            let actual = "".to_string();
            let expected = "{{anyNonEmptyString}}".to_string();
            let stub = ResponseStub {
                body: BodyStub { body: Some(expected.clone()), ..Default::default() },
                transformers: vec![String::from("response-template")],
                ..Default::default()
            };
            let req = Request::get("http://localhost");
            let mut resp = Response::new(200);
            resp.set_body(actual.as_str());
            TextBodyTemplatingVerifier { actual, expected }
                .verify(&stub, "blank", &mut StdRequest(req), &mut StdResponse(resp));
        }

        #[should_panic(expected = "Verification failed for stub 'blank'. Expected text response body to match '{{anyNonEmptyString}}' but no response body was present")]
        #[test]
        fn verify_text_body_should_fail_when_body_absent() {
            let expected = "{{anyNonEmptyString}}".to_string();
            let stub = ResponseStub {
                body: BodyStub { body: Some(expected.clone()), ..Default::default() },
                transformers: vec![String::from("response-template")],
                ..Default::default()
            };
            let req = Request::get("http://localhost");
            TextBodyTemplatingVerifier { actual: String::new(), expected }
                .verify(&stub, "blank", &mut StdRequest(req), &mut StdResponse(Response::new(200)));
        }
    }
}
