use crate::model::response::{
    template::{
        data::{HandlebarsData, RequestData},
        verify::Predictable,
        HandlebarTemplatable,
    },
    ResponseStub,
};

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
            assert_eq!(
                self.actual, expected,
                "\nVerification failed for stub '{}'. Expected response body to be '{}' but was '{}'",
                name, expected, self.actual
            );
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
                body: BodyStub {
                    body: Some(expected.clone()),
                    ..Default::default()
                },
                transformers: vec![String::from("response-template")],
                ..Default::default()
            };
            let mut req = Request::get(format!("http://localhost{}", &actual).as_str());
            let mut resp = Response::new(200);
            resp.set_body(actual.as_str());
            TextBodyTemplatingVerifier { actual, expected }.verify(&stub, "text", &RequestData::from(&mut req), &mut StdResponse(resp));
        }

        #[test]
        fn should_verify_text_body_when_many_templates() {
            let actual = "one-two-three".to_string();
            let expected = "{{request.pathSegments.[0]}}-{{request.pathSegments.[1]}}-{{request.pathSegments.[2]}}".to_string();
            let stub = ResponseStub {
                body: BodyStub {
                    body: Some(expected.clone()),
                    ..Default::default()
                },
                transformers: vec![String::from("response-template")],
                ..Default::default()
            };
            let mut req = Request::get("http://localhost/one/two/three");
            let mut resp = Response::new(200);
            resp.set_body(actual.as_str());
            TextBodyTemplatingVerifier { actual, expected }.verify(&stub, "text", &RequestData::from(&mut req), &mut StdResponse(resp));
        }

        #[should_panic(
            expected = "Verification failed for stub 'text'. Expected response body to be 'three-two-one' but was 'one-two-three'"
        )]
        #[test]
        fn verify_text_body_should_fail_when_not_eq() {
            let actual = "one-two-three".to_string();
            let expected = "{{request.pathSegments.[2]}}-{{request.pathSegments.[1]}}-{{request.pathSegments.[0]}}".to_string();
            let stub = ResponseStub {
                body: BodyStub {
                    body: Some(expected.clone()),
                    ..Default::default()
                },
                transformers: vec![String::from("response-template")],
                ..Default::default()
            };
            let mut req = Request::get("http://localhost/one/two/three");
            let mut resp = Response::new(200);
            resp.set_body(actual.as_str());
            TextBodyTemplatingVerifier { actual, expected }.verify(&stub, "text", &RequestData::from(&mut req), &mut StdResponse(resp));
        }
    }

    mod any {
        use super::*;

        #[should_panic(
            expected = "Cannot verify stub 'text' because response body '{{anyNonBlankString}}-{{anyNonEmptyString}}' is not verifiable"
        )]
        #[test]
        fn verify_text_body_when_many_rnd_template_should_not_verify_because_not_predictable() {
            verify("text", "anything", "{{anyNonBlankString}}-{{anyNonEmptyString}}")
        }
    }

    mod any_of {
        use super::*;

        #[test]
        fn should_verify_body() {
            verify("enum", "A", "{{anyOf 'A' 'B' 'C'}}");
            verify("enum", "B", "{{anyOf 'A' 'B' 'C'}}");
            verify("enum", "C", "{{anyOf 'A' 'B' 'C'}}");
        }

        #[should_panic(
            expected = "Verification failed for stub 'enum'. Expected response body to be one of [\"A\", \"B\", \"C\"] but was 'D'"
        )]
        #[test]
        fn verify_body_should_fail_when_regex_does_not_match() {
            verify("enum", "D", "{{anyOf 'A' 'B' 'C'}}");
        }

        #[should_panic(
            expected = "Verification failed for stub 'enum'. Expected response body to be one of [\"A\", \"B\", \"C\"] but no response body was present"
        )]
        #[test]
        fn verify_body_should_fail_when_body_absent() {
            verify("enum", "", "{{anyOf 'A' 'B' 'C'}}")
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

        #[should_panic(
            expected = "Verification failed for stub 'regex'. Expected response body to match '[A-Z]{2}' but no response body was present"
        )]
        #[test]
        fn verify_body_should_fail_when_body_absent() {
            verify("regex", "", "{{anyRegex '[A-Z]{2}'}}")
        }
    }

    mod any_uuid {
        use super::*;

        #[test]
        fn should_verify_body() {
            verify("uuid", "6a2f41a3-c54c-fce8-32d2-0324e1c32e22", "{{anyUuid}}")
        }

        #[should_panic(expected = "Verification failed for stub 'uuid'. Expected response body to be a valid uuid but was 'abcd'")]
        #[test]
        fn verify_body_should_fail_when_uuid_does_not_match() {
            verify("uuid", "abcd", "{{anyUuid}}")
        }

        #[should_panic(
            expected = "Verification failed for stub 'uuid'. Expected response body to be a valid uuid but was '6a2f41a3-c54c-fce8-32d2-0324e1c32e22-43a1'"
        )]
        #[test]
        fn verify_body_should_fail_when_uuid_longer() {
            verify("uuid", "6a2f41a3-c54c-fce8-32d2-0324e1c32e22-43a1", "{{anyUuid}}")
        }

        #[should_panic(
            expected = "Verification failed for stub 'uuid'. Expected response body to be a valid uuid but no response body was present"
        )]
        #[test]
        fn verify_body_should_fail_when_body_absent() {
            verify("uuid", "", "{{anyUuid}}")
        }
    }

    mod any_email {
        use super::*;

        #[test]
        fn should_verify_body() {
            verify("email", "john.doe@gmail.com", "{{anyEmail}}")
        }

        #[should_panic(
            expected = "Verification failed for stub 'email'. Expected response body to be a valid email address but was 'john'"
        )]
        #[test]
        fn verify_body_should_fail_when_email_does_not_match() {
            verify("email", "john", "{{anyEmail}}")
        }

        #[should_panic(
            expected = "Verification failed for stub 'email'. Expected response body to be a valid email address but no response body was present"
        )]
        #[test]
        fn verify_body_should_fail_when_body_absent() {
            verify("email", "", "{{anyEmail}}")
        }
    }

    mod any_hostname {
        use super::*;

        #[test]
        fn should_verify_body() {
            verify("host", "https://github.com", "{{anyHostname}}")
        }

        #[should_panic(
            expected = "Verification failed for stub 'host'. Expected response body to be a valid hostname but was 'github.com'"
        )]
        #[test]
        fn verify_body_should_fail_when_host_does_not_match() {
            verify("host", "github.com", "{{anyHostname}}")
        }

        #[should_panic(
            expected = "Verification failed for stub 'host'. Expected response body to be a valid hostname but no response body was present"
        )]
        #[test]
        fn verify_body_should_fail_when_body_absent() {
            verify("host", "", "{{anyHostname}}")
        }
    }

    mod any_ip {
        use super::*;

        #[test]
        fn should_verify_body() {
            verify("ip", "127.0.0.1", "{{anyIpAddress}}")
        }

        #[should_panic(expected = "Verification failed for stub 'ip'. Expected response body to be a valid ip address but was '127.0.0'")]
        #[test]
        fn verify_body_should_fail_when_ip_does_not_match() {
            verify("ip", "127.0.0", "{{anyIpAddress}}")
        }

        #[should_panic(
            expected = "Verification failed for stub 'ip'. Expected response body to be a valid ip address but no response body was present"
        )]
        #[test]
        fn verify_body_should_fail_when_body_absent() {
            verify("ip", "", "{{anyIpAddress}}")
        }
    }

    mod any_bool {
        use super::*;

        #[test]
        fn should_verify_body() {
            verify("bool", "true", "{{anyBoolean}}");
            verify("bool", "false", "{{anyBoolean}}");
        }

        #[should_panic(expected = "Verification failed for stub 'bool'. Expected response body to be a boolean but was 'either'")]
        #[test]
        fn verify_body_should_fail_when_bool_does_not_match() {
            verify("bool", "either", "{{anyBoolean}}")
        }

        #[should_panic(
            expected = "Verification failed for stub 'bool'. Expected response body to be a boolean but no response body was present"
        )]
        #[test]
        fn verify_body_should_fail_when_body_absent() {
            verify("bool", "", "{{anyBoolean}}")
        }
    }

    mod any_date {
        use super::*;

        #[test]
        fn should_verify_body() {
            verify("date", "2022-04-13", "{{anyDate}}")
        }

        #[should_panic(
            expected = "Verification failed for stub 'date'. Expected response body to be a valid date (yyyy-mm-dd) but was '2022/04/13'"
        )]
        #[test]
        fn verify_body_should_fail_when_date_does_not_match() {
            verify("date", "2022/04/13", "{{anyDate}}")
        }

        #[should_panic(
            expected = "Verification failed for stub 'date'. Expected response body to be a valid date (yyyy-mm-dd) but no response body was present"
        )]
        #[test]
        fn verify_body_should_fail_when_body_absent() {
            verify("date", "", "{{anyDate}}")
        }
    }

    mod any_time {
        use super::*;

        #[test]
        fn should_verify_body() {
            verify("time", "23:59:59", "{{anyTime}}")
        }

        #[should_panic(
            expected = "Verification failed for stub 'time'. Expected response body to be a valid time (hh:mm:ss) but was '24:59:59'"
        )]
        #[test]
        fn verify_body_should_fail_when_time_does_not_match() {
            verify("time", "24:59:59", "{{anyTime}}")
        }

        #[should_panic(
            expected = "Verification failed for stub 'time'. Expected response body to be a valid time (hh:mm:ss) but no response body was present"
        )]
        #[test]
        fn verify_body_should_fail_when_body_absent() {
            verify("time", "", "{{anyTime}}")
        }
    }

    mod any_datetime {
        use super::*;

        #[test]
        fn should_verify_body() {
            verify("datetime", "2022-04-13T23:59:59", "{{anyDatetime}}")
        }

        #[should_panic(
            expected = "Verification failed for stub 'datetime'. Expected response body to be a valid datetime (yyyy-mm-ddThh:mm:ss) but was '2022/04/13T24:59:59'"
        )]
        #[test]
        fn verify_body_should_fail_when_datetime_does_not_match() {
            verify("datetime", "2022/04/13T24:59:59", "{{anyDatetime}}")
        }

        #[should_panic(
            expected = "Verification failed for stub 'datetime'. Expected response body to be a valid datetime (yyyy-mm-ddThh:mm:ss) but no response body was present"
        )]
        #[test]
        fn verify_body_should_fail_when_body_absent() {
            verify("datetime", "", "{{anyDatetime}}")
        }
    }

    mod any_iso_8601_datetime {
        use super::*;

        #[test]
        fn should_verify_body() {
            verify("iso-8601-datetime", "2022-04-13T23:59:59Z", "{{anyIso8601}}");
            verify("iso-8601-datetime", "2022-04-13T23:59:59+01:00", "{{anyIso8601}}");
        }

        #[should_panic(
            expected = "Verification failed for stub 'iso-8601-datetime'. Expected response body to be a valid iso 8601 datetime (yyyy-mm-ddThh:mm:ss) but was '2022/04/13T24:59:59'"
        )]
        #[test]
        fn verify_body_should_fail_when_iso_8601_datetime_does_not_match() {
            verify("iso-8601-datetime", "2022/04/13T24:59:59", "{{anyIso8601}}")
        }

        #[should_panic(
            expected = "Verification failed for stub 'iso-8601-datetime'. Expected response body to be a valid iso 8601 datetime (yyyy-mm-ddThh:mm:ss) but no response body was present"
        )]
        #[test]
        fn verify_body_should_fail_when_body_absent() {
            verify("iso-8601-datetime", "", "{{anyIso8601}}")
        }
    }

    mod any_non_blank_string {
        use super::*;

        #[test]
        fn should_verify_body() {
            verify("blank", "azerty", "{{anyNonBlankString}}")
        }

        #[should_panic(expected = "Verification failed for stub 'blank'. Expected response body to be a non blank string but was ' '")]
        #[test]
        fn verify_body_should_fail_when_body_contains_space() {
            verify("blank", " ", "{{anyNonBlankString}}")
        }

        #[should_panic(
            expected = "Verification failed for stub 'blank'. Expected response body to be a non blank string but no response body was present"
        )]
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

        #[should_panic(
            expected = "Verification failed for stub 'empty'. Expected response body to be a non empty string but no response body was present"
        )]
        #[test]
        fn verify_body_should_fail_when_body_empty() {
            verify("empty", "", "{{anyNonEmptyString}}")
        }

        #[should_panic(
            expected = "Verification failed for stub 'empty'. Expected response body to be a non empty string but no response body was present"
        )]
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

        #[should_panic(expected = "Verification failed for stub 'alpha-num'. Expected response body to be an alphanumeric but was '!?'")]
        #[test]
        fn verify_body_should_fail_when_not_alpha_numeric() {
            verify("alpha-num", "!?", "{{anyAlphaNumeric}}")
        }

        #[should_panic(
            expected = "Verification failed for stub 'alpha-num'. Expected response body to be an alphanumeric but no response body was present"
        )]
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

        #[should_panic(expected = "Verification failed for stub 'number'. Expected response body to be a number but was 'abcd'")]
        #[test]
        fn verify_body_should_fail_when_not_number() {
            verify("number", "abcd", "{{anyNumber}}")
        }

        #[should_panic(
            expected = "Verification failed for stub 'number'. Expected response body to be a number but no response body was present"
        )]
        #[test]
        fn verify_body_should_fail_when_body_absent() {
            verify("number", "", "{{anyNumber}}")
        }
    }

    mod any_integer {
        use super::*;

        #[test]
        fn should_verify_body() {
            verify("integer", "42", "{{anyI64}}");
        }

        #[should_panic(expected = "Verification failed for stub 'integer'. Expected response body to be an i64 but was 'abcd'")]
        #[test]
        fn verify_body_should_fail_when_not_integer() {
            verify("integer", "abcd", "{{anyI64}}")
        }

        #[should_panic(expected = "Verification failed for stub 'integer'. Expected response body to be an i64 but was '42.3'")]
        #[test]
        fn verify_body_should_fail_when_float() {
            verify("integer", "42.3", "{{anyI64}}")
        }

        #[should_panic(
            expected = "Verification failed for stub 'integer'. Expected response body to be an i64 but no response body was present"
        )]
        #[test]
        fn verify_body_should_fail_when_body_absent() {
            verify("integer", "", "{{anyI64}}")
        }
    }

    mod any_float {
        use super::*;

        #[test]
        fn should_verify_body() {
            verify("float", "42.3", "{{anyFloat}}");
        }

        #[should_panic(expected = "Verification failed for stub 'float'. Expected response body to be a float but was 'abcd'")]
        #[test]
        fn verify_body_should_fail_when_not_float() {
            verify("float", "abcd", "{{anyFloat}}")
        }

        #[should_panic(expected = "Verification failed for stub 'float'. Expected response body to be a float but was '42'")]
        #[test]
        fn verify_body_should_fail_when_integer() {
            verify("float", "42", "{{anyFloat}}")
        }

        #[should_panic(
            expected = "Verification failed for stub 'float'. Expected response body to be a float but no response body was present"
        )]
        #[test]
        fn verify_body_should_fail_when_body_absent() {
            verify("float", "", "{{anyFloat}}")
        }
    }

    fn verify(name: &str, actual: &str, expected: &str) {
        let stub = ResponseStub {
            body: BodyStub {
                body: Some(expected.to_string()),
                ..Default::default()
            },
            transformers: vec![String::from("response-template")],
            ..Default::default()
        };
        let mut resp = Response::new(200);
        resp.set_body(actual);
        TextBodyTemplatingVerifier {
            actual: actual.to_string(),
            expected: expected.to_string(),
        }
        .verify(
            &stub,
            name,
            &RequestData::from(&mut Request::get("http://localhost")),
            &mut StdResponse(resp),
        );
    }
}
