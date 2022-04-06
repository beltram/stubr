use crate::model::response::{ResponseStub, template::{data::{HandlebarsData, RequestData}, HandlebarTemplatable, verify::Predictable}};

use super::super::{StdResponse, Verifier};

pub struct TextBodyTemplatingVerifier {
    pub actual: String,
    pub expected: String,
}

impl Verifier<'_> for TextBodyTemplatingVerifier {
    fn verify(self, stub: &'_ ResponseStub, name: &'_ str, req: &'_ RequestData, _: &'_ mut StdResponse) {
        let data = HandlebarsData {
            request: req,
            response: Some(self.actual.as_bytes()),
            is_verify: true,
            stub_name: Some(name),
        };
        stub.body.register(&self.expected, &self.expected);
        let expected = stub.body.render(&self.expected, &data);
        if self.expected.is_predictable() {
            assert_eq!(self.actual, expected, "\nVerification failed for stub '{}'. Expected response body to be '{}' but was '{}'", name, expected, self.actual);
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
            let mut req = Request::get(format!("http://localhost{}", &actual).as_str());
            let mut resp = Response::new(200);
            resp.set_body(actual.as_str());
            TextBodyTemplatingVerifier { actual, expected }
                .verify(&stub, "text", &RequestData::from(&mut req), &mut StdResponse(resp));
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
            let mut req = Request::get("http://localhost/one/two/three");
            let mut resp = Response::new(200);
            resp.set_body(actual.as_str());
            TextBodyTemplatingVerifier { actual, expected }
                .verify(&stub, "text", &RequestData::from(&mut req), &mut StdResponse(resp));
        }

        #[should_panic(expected = "Verification failed for stub 'text'. Expected response body to be 'three-two-one' but was 'one-two-three'")]
        #[test]
        fn verify_text_body_should_fail_when_not_eq() {
            let actual = "one-two-three".to_string();
            let expected = "{{request.pathSegments.[2]}}-{{request.pathSegments.[1]}}-{{request.pathSegments.[0]}}".to_string();
            let stub = ResponseStub {
                body: BodyStub { body: Some(expected.clone()), ..Default::default() },
                transformers: vec![String::from("response-template")],
                ..Default::default()
            };
            let mut req = Request::get("http://localhost/one/two/three");
            let mut resp = Response::new(200);
            resp.set_body(actual.as_str());
            TextBodyTemplatingVerifier { actual, expected }
                .verify(&stub, "text", &RequestData::from(&mut req), &mut StdResponse(resp));
        }
    }

    mod any {
        use super::*;

        #[should_panic(expected = "Cannot verify stub 'text' because response body '{{anyNonBlankString}}-{{anyNonEmptyString}}' is not verifiable")]
        #[test]
        fn verify_text_body_when_many_rnd_template_should_not_verify_because_not_predictable() {
            verify(
                "text",
                "anything",
                "{{anyNonBlankString}}-{{anyNonEmptyString}}",
            )
        }
    }

    mod any_regex {
        use super::*;

        #[test]
        fn should_verify_body() {
            verify("regex", "FR", "{{anyRegex '[A-Z]{2}'}}")
        }

        #[should_panic(expected = "Verification failed for stub 'regex'. Expected response body to match '[A-Z]{2}' but was 'fr'")]
        #[test]
        fn verify_body_should_fail_when_regex_does_not_match() {
            verify("regex", "fr", "{{anyRegex '[A-Z]{2}'}}")
        }

        #[should_panic(expected = "Verification failed for stub 'regex'. Expected response body to match '[A-Z]{2}' but no response body was present")]
        #[test]
        fn verify_body_should_fail_when_body_absent() {
            verify("regex", "", "{{anyRegex '[A-Z]{2}'}}")
        }
    }

    mod any_non_blank_string {
        use super::*;

        #[test]
        fn should_verify_body() {
            verify("blank", "azerty", "{{anyNonBlankString}}")
        }

        #[should_panic(expected = "Verification failed for stub 'blank'. Expected response body to match '{{anyNonBlankString}}' but was ' '")]
        #[test]
        fn verify_body_should_fail_when_body_contains_space() {
            verify("blank", " ", "{{anyNonBlankString}}")
        }

        #[should_panic(expected = "Verification failed for stub 'blank'. Expected response body to match '{{anyNonBlankString}}' but no response body was present")]
        #[test]
        fn verify_body_should_fail_when_body_absent() {
            verify("blank", "", "{{anyNonBlankString}}")
        }
    }

    mod any_non_empty_string {
        use super::*;

        #[test]
        fn should_verify_body() {
            verify("empty", "azerty", "{{anyNonEmptyString}}")
        }

        #[should_panic(expected = "Verification failed for stub 'empty'. Expected response body to match '{{anyNonEmptyString}}' but no response body was present")]
        #[test]
        fn verify_body_should_fail_when_body_empty() {
            verify("empty", "", "{{anyNonEmptyString}}")
        }

        #[should_panic(expected = "Verification failed for stub 'empty'. Expected response body to match '{{anyNonEmptyString}}' but no response body was present")]
        #[test]
        fn verify_body_should_fail_when_body_absent() {
            verify("empty", "", "{{anyNonEmptyString}}")
        }
    }

    mod any_alpha_numeric {
        use super::*;

        #[test]
        fn should_verify_body() {
            verify("alpha-num", "abcd1234ABCD", "{{anyAlphaNumeric}}")
        }

        #[should_panic(expected = "Verification failed for stub 'alpha-num'. Expected response body to match '{{anyAlphaNumeric}}' but was '!?'")]
        #[test]
        fn verify_body_should_fail_when_not_alpha_numeric() {
            verify("alpha-num", "!?", "{{anyAlphaNumeric}}")
        }

        #[should_panic(expected = "Verification failed for stub 'alpha-num'. Expected response body to match '{{anyAlphaNumeric}}' but no response body was present")]
        #[test]
        fn verify_body_should_fail_when_body_absent() {
            verify("alpha-num", "", "{{anyAlphaNumeric}}")
        }
    }

    mod any_number {
        use super::*;

        #[test]
        fn should_verify_body() {
            verify("number", "42", "{{anyNumber}}");
            verify("number", "42.3", "{{anyNumber}}");
        }

        #[should_panic(expected = "Verification failed for stub 'number'. Expected response body to match '{{anyNumber}}' but was 'abcd'")]
        #[test]
        fn verify_body_should_fail_when_not_number() {
            verify("number", "abcd", "{{anyNumber}}")
        }

        #[should_panic(expected = "Verification failed for stub 'number'. Expected response body to match '{{anyNumber}}' but no response body was present")]
        #[test]
        fn verify_body_should_fail_when_body_absent() {
            verify("number", "", "{{anyNumber}}")
        }
    }

    mod any_integer {
        use super::*;

        #[test]
        fn should_verify_body() {
            verify("integer", "42", "{{anyInt}}");
        }

        #[should_panic(expected = "Verification failed for stub 'integer'. Expected response body to match '{{anyInt}}' but was 'abcd'")]
        #[test]
        fn verify_body_should_fail_when_not_integer() {
            verify("integer", "abcd", "{{anyInt}}")
        }

        #[should_panic(expected = "Verification failed for stub 'integer'. Expected response body to match '{{anyInt}}' but was '42.3'")]
        #[test]
        fn verify_body_should_fail_when_float() {
            verify("integer", "42.3", "{{anyInt}}")
        }

        #[should_panic(expected = "Verification failed for stub 'integer'. Expected response body to match '{{anyInt}}' but no response body was present")]
        #[test]
        fn verify_body_should_fail_when_body_absent() {
            verify("integer", "", "{{anyInt}}")
        }
    }

    mod any_float {
        use super::*;

        #[test]
        fn should_verify_body() {
            verify("float", "42.3", "{{anyFloat}}");
        }

        #[should_panic(expected = "Verification failed for stub 'float'. Expected response body to match '{{anyFloat}}' but was 'abcd'")]
        #[test]
        fn verify_body_should_fail_when_not_float() {
            verify("float", "abcd", "{{anyFloat}}")
        }

        #[should_panic(expected = "Verification failed for stub 'float'. Expected response body to match '{{anyFloat}}' but was '42'")]
        #[test]
        fn verify_body_should_fail_when_integer() {
            verify("float", "42", "{{anyFloat}}")
        }

        #[should_panic(expected = "Verification failed for stub 'float'. Expected response body to match '{{anyFloat}}' but no response body was present")]
        #[test]
        fn verify_body_should_fail_when_body_absent() {
            verify("float", "", "{{anyFloat}}")
        }
    }

    fn verify(name: &str, actual: &str, expected: &str) {
        let stub = ResponseStub {
            body: BodyStub { body: Some(expected.to_string()), ..Default::default() },
            transformers: vec![String::from("response-template")],
            ..Default::default()
        };
        let mut resp = Response::new(200);
        resp.set_body(actual);
        TextBodyTemplatingVerifier { actual: actual.to_string(), expected: expected.to_string() }
            .verify(&stub, name, &RequestData::from(&mut Request::get("http://localhost")), &mut StdResponse(resp));
    }
}
