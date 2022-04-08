use serde_json::Value;

use crate::model::response::{ResponseStub, template::data::RequestData};

use super::super::{StdResponse, Verifier};

mod object;
mod string;

pub struct JsonBodyTemplatingVerifier {
    pub actual: Value,
    pub expected: Value,
}

impl Verifier<'_> for JsonBodyTemplatingVerifier {
    fn verify(self, stub: &'_ ResponseStub, name: &'_ str, req: &'_ RequestData, resp: &'_ mut StdResponse) {
        if let Ok(object_verifier) = object::JsonObjectVerifier::try_from(&self) {
            object_verifier.verify(stub, name, req, resp)
        } else if let Some((actual, expected)) = self.actual.as_array().zip(self.expected.as_array()) {
            assert!(actual.len().ge(&expected.len()),
                    "Verification failed for stub '{}'. Expected {} elements in json response body but {} found",
                    name, expected.len(), actual.len());
            actual.iter().zip(expected.iter())
                .for_each(|(a, e)| Self { actual: a.clone(), expected: e.clone() }.verify(stub, name, req, resp))
        } else if let Ok(str_verifier) = string::JsonStrVerifier::try_from(&self) {
            str_verifier.verify(stub, name, req, resp)
        } else {
            assert_eq!(self.actual, self.expected,
                       "Verification failed for stub '{}'. Expected json response body to be '{}' but was '{}'",
                       name, self.expected, self.actual)
        }
    }
}

#[cfg(test)]
mod json_body_verify_tests {
    use http_types::{Request, Response};
    use serde_json::json;

    use crate::model::response::body::BodyStub;

    use super::*;

    mod from_req {
        use super::*;

        #[test]
        fn should_verify_json_body() {
            verify(
                "json",
                json!({"name": "alice"}),
                json!({"name": "{{jsonPath request.body '$.name'}}"}),
            )
        }

        #[test]
        fn should_verify_many_json_body() {
            verify(
                "json",
                json!({"a": "alice", "b": "wonderland"}),
                json!({"a": "{{jsonPath request.body '$.a'}}", "b": "{{jsonPath request.body '$.b'}}"}),
            )
        }

        #[test]
        fn should_verify_json_body_alongside_unpredictable() {
            verify(
                "json",
                json!({"id": 23, "name": "alice"}),
                json!({"id": "{{anyI64}}", "name": "{{jsonPath request.body '$.name'}}"}),
            )
        }

        #[test]
        fn should_verify_json_array_body() {
            verify(
                "json",
                json!(["alice"]),
                json!(["{{jsonPath request.body '$.name'}}"]),
            )
        }

        #[should_panic(expected = "Verification failed for stub 'json'. Expected json response body for field 'name' to be 'alice' but was '\"bob\"'")]
        #[test]
        fn should_fail_verifying_json_when_not_eq() {
            let actual = json!({"name": "bob"});
            let expected = json!({"name": "{{jsonPath request.body '$.name'}}"});
            let stub = stub(&expected);
            let mut req = Request::post("http://localhost/");
            req.set_body(json!({"name": "alice"}));
            let mut resp = Response::new(200);
            resp.set_body(actual.clone());
            JsonBodyTemplatingVerifier { actual, expected }
                .verify(&stub, "json", &RequestData::from(&mut req), &mut StdResponse(resp));
        }

        #[test]
        fn should_verify_json_body_from_path_segments() {
            let id = 1;
            verify_with_uri(
                "json",
                json!({"id": id}),
                json!({"id": "{{request.pathSegments.[0]}}"}),
                &format!("http://localhost/{}", id),
            )
        }
    }

    mod any {
        use super::*;

        mod obj {
            use super::*;

            #[test]
            fn should_verify_json_regardless_json_obj_key_order() {
                verify(
                    "any",
                    json!({"name": "john", "country": "FR"}),
                    json!({"name": "{{anyNonBlankString}}", "country": "{{anyNonBlankString}}"}),
                )
            }

            #[test]
            fn should_verify_nested_templated() {
                verify(
                    "any",
                    json!({"country": {"name": "FR", "population": 69000000}}),
                    json!({"country": {"name": "{{anyNonBlankString}}", "population": 69000000}}),
                )
            }

            #[should_panic(expected = "Verification failed for stub 'any'. Expected response body to match '{{anyNonBlankString}}' but was ' '")]
            #[test]
            fn verify_nested_templated_should_fail_when_not_eq() {
                verify(
                    "any",
                    json!({"country": {"name": " ", "population": 69000000}}),
                    json!({"country": {"name": "{{anyNonBlankString}}", "population": 69000000}}),
                )
            }

            #[test]
            fn should_verify_json_for_not_templated_values() {
                verify(
                    "any",
                    json!({"name": "john", "country": "FR"}),
                    json!({"name": "{{anyNonBlankString}}", "country": "FR"}),
                )
            }

            #[should_panic(expected = "Verification failed for stub 'any'. Expected json field to be 'UK' but was 'FR'")]
            #[test]
            fn verify_json_should_fail_when_not_templated_not_eq() {
                verify(
                    "any",
                    json!({"name": "john", "country": "FR"}),
                    json!({"name": "{{anyNonBlankString}}", "country": "UK"}),
                )
            }

            #[should_panic(expected = "Verification failed for stub 'any'. Expected json field to be 'UK' but was 'FR'")]
            #[test]
            fn verify_json_should_fail_when_not_templated_obj_not_eq() {
                verify(
                    "any",
                    json!({"name": "john", "country": {"name": "FR"}}),
                    json!({"name": "{{anyNonBlankString}}", "country": {"name": "UK"}}),
                )
            }

            #[should_panic(expected = "Verification failed for stub 'any'. Expected json fields '[(\"country\", \"{{anyRegex '[A-Z]{2}'}}\")]' were absent from response body")]
            #[test]
            fn verify_json_fail_when_keys_mismatch() {
                verify(
                    "any",
                    json!({"name": "john", "pays": "FR"}),
                    json!({"name": "{{anyRegex '[a-z]+'}}", "country": "{{anyRegex '[A-Z]{2}'}}"}),
                )
            }

            #[test]
            fn verify_json_should_not_fail_when_interleaved_key() {
                verify(
                    "any",
                    json!({"name": "john", "age": 42, "country": "FR"}),
                    json!({"name": "{{anyRegex '[a-z]+'}}", "country": "{{anyRegex '[A-Z]{2}'}}"}),
                )
            }

            #[should_panic(expected = "Verification failed for stub 'any'. Expected json fields '[(\"country\", \"{{anyRegex '[A-Z]{2}'}}\")]' were absent from response body")]
            #[test]
            fn verify_json_should_fail_when_more_keys_expected_than_present() {
                verify(
                    "any",
                    json!({"name": "john"}),
                    json!({"name": "{{anyRegex '[a-z]+'}}", "country": "{{anyRegex '[A-Z]{2}'}}"}),
                )
            }

            #[test]
            fn should_verify_when_both_empty() {
                verify("any", json!({}), json!({}));
            }
        }

        mod array {
            use super::*;

            #[test]
            fn should_verify_json_arrays() {
                verify("any", json!(["alice"]), json!(["{{anyNonBlankString}}"]))
            }

            #[should_panic(expected = "Verification failed for stub 'any'. Expected response body to match '{{anyNonBlankString}}' but was ' '")]
            #[test]
            fn verify_json_arrays_should_fail_when_not_match() {
                verify("any", json!([" "]), json!(["{{anyNonBlankString}}"]))
            }

            #[test]
            fn verify_json_arrays_should_not_fail_when_more_than_expected() {
                verify("any", json!(["alice", "bob"]), json!(["{{anyNonBlankString}}"]))
            }

            #[should_panic(expected = "Verification failed for stub 'any'. Expected 2 elements in json response body but 1 found")]
            #[test]
            fn verify_json_arrays_should_fail_expected_absent() {
                verify("any", json!(["alice"]), json!(["{{anyNonBlankString}}", "{{anyNonBlankString}}"]))
            }

            #[test]
            fn should_verify_when_both_empty() {
                verify("any", json!([]), json!([]));
            }
        }

        #[should_panic(expected = "Verification failed for stub 'any'. Expected json response body to be '{\"name\":\"{{anyNonBlankString}}\"}' but was '[\"alice\"]'")]
        #[test]
        fn verify_should_fail_when_different_types() {
            verify(
                "any",
                json!(["alice"]),
                json!({"name": "{{anyNonBlankString}}"}),
            )
        }
    }

    mod types {
        use super::*;

        mod string {
            use super::*;

            #[test]
            fn should_verify_json_partially() {
                verify("regex", json!({"country": "FR"}), json!({"country": "{{anyRegex '^[A-Z]{2}$'}}"}))
            }

            #[should_panic(expected = "Verification failed for stub 'regex'. Expected response body to match '^[A-Z]{2}$' but was 'FRANCE'")]
            #[test]
            fn verify_json_partially_should_fail() {
                verify("regex", json!({"country": "FRANCE"}), json!({"country": "{{anyRegex '^[A-Z]{2}$'}}"}))
            }

            #[should_panic(expected = "Verification failed for stub 'regex'. Expected response body to match '^[A-Z]{2}$' but was '42'")]
            #[test]
            fn verify_json_partially_should_fail_when_not_string() {
                verify("regex", json!({"country": 42}), json!({"country": "{{anyRegex '^[A-Z]{2}$'}}"}))
            }
        }

        mod number {
            use super::*;

            #[test]
            fn should_verify_json_number_partially() {
                verify("number", json!({"age": 42}), json!({"age": "{{anyNumber}}"}));
                verify("number", json!({"age": 42.3}), json!({"age": "{{anyNumber}}"}));
            }

            #[should_panic(expected = "Verification failed for stub 'number'. Expected response body to match '{{anyNumber}}' but was 'abcd'")]
            #[test]
            fn verify_json_number_partially_should_fail() {
                verify("number", json!({"age": "abcd"}), json!({"age": "{{anyNumber}}"}))
            }
        }

        mod int {
            use super::*;

            mod i64 {
                use super::*;

                #[test]
                fn should_verify_json_i64_partially() {
                    verify("i64", json!({"age": i64::MAX}), json!({"age": "{{anyI64}}"}));
                    verify("i64", json!({"age": i64::MIN}), json!({"age": "{{anyI64}}"}));
                }

                #[should_panic(expected = "Verification failed for stub 'i64'. Expected response body to match '{{anyI64}}' but was '42.3'")]
                #[test]
                fn verify_json_i64_partially_should_fail_when_float() {
                    verify("i64", json!({"age": 42.3}), json!({"age": "{{anyI64}}"}))
                }

                #[should_panic(expected = "Verification failed for stub 'i64'. Expected response body to match '{{anyI64}}' but was 'abcd'")]
                #[test]
                fn verify_json_i64_partially_should_fail_when_string() {
                    verify("i64", json!({"age": "abcd"}), json!({"age": "{{anyI64}}"}))
                }

                #[should_panic(expected = "Verification failed for stub 'i64'. Expected response body to match '{{anyI64}}' but was '18446744073709551615'")]
                #[test]
                fn verify_json_i64_partially_should_fail_when_too_large() {
                    verify("i64", json!({"age": u64::MAX}), json!({"age": "{{anyI64}}"}))
                }
            }

            mod u64 {
                use super::*;

                #[test]
                fn should_verify_json_u64_partially() {
                    verify("u64", json!({"age": u64::MAX}), json!({"age": "{{anyU64}}"}));
                    verify("u64", json!({"age": u64::MIN}), json!({"age": "{{anyU64}}"}));
                }

                #[should_panic(expected = "Verification failed for stub 'u64'. Expected response body to match '{{anyU64}}' but was '42.3'")]
                #[test]
                fn verify_json_u64_partially_should_fail_when_float() {
                    verify("u64", json!({"age": 42.3}), json!({"age": "{{anyU64}}"}))
                }

                #[should_panic(expected = "Verification failed for stub 'u64'. Expected response body to match '{{anyU64}}' but was 'abcd'")]
                #[test]
                fn verify_json_u64_partially_should_fail_when_string() {
                    verify("u64", json!({"age": "abcd"}), json!({"age": "{{anyU64}}"}))
                }

                #[should_panic(expected = "Verification failed for stub 'u64'. Expected response body to match '{{anyU64}}' but was '-9223372036854775808'")]
                #[test]
                fn verify_json_u64_partially_should_fail_when_negative() {
                    verify("u64", json!({"age": i64::MIN}), json!({"age": "{{anyU64}}"}))
                }
            }

            mod i32 {
                use super::*;

                #[test]
                fn should_verify_json_i32_partially() {
                    verify("i32", json!({"age": i32::MAX}), json!({"age": "{{anyI32}}"}));
                    verify("i32", json!({"age": i32::MIN}), json!({"age": "{{anyI32}}"}));
                }

                #[should_panic(expected = "Verification failed for stub 'i32'. Expected response body to match '{{anyI32}}' but was '42.3'")]
                #[test]
                fn verify_json_i32_partially_should_fail_when_float() {
                    verify("i32", json!({"age": 42.3}), json!({"age": "{{anyI32}}"}))
                }

                #[should_panic(expected = "Verification failed for stub 'i32'. Expected response body to match '{{anyI32}}' but was 'abcd'")]
                #[test]
                fn verify_json_i32_partially_should_fail_when_string() {
                    verify("i32", json!({"age": "abcd"}), json!({"age": "{{anyI32}}"}))
                }

                #[should_panic(expected = "Verification failed for stub 'i32'. Expected response body to match '{{anyI32}}' but was '4294967295'")]
                #[test]
                fn verify_json_i32_partially_should_fail_when_too_large() {
                    verify("i32", json!({"age": u32::MAX}), json!({"age": "{{anyI32}}"}))
                }
            }

            mod u32 {
                use super::*;

                #[test]
                fn should_verify_json_u32_partially() {
                    verify("u32", json!({"age": u32::MAX}), json!({"age": "{{anyU32}}"}));
                    verify("u32", json!({"age": u32::MIN}), json!({"age": "{{anyU32}}"}));
                }

                #[should_panic(expected = "Verification failed for stub 'u32'. Expected response body to match '{{anyU32}}' but was '42.3'")]
                #[test]
                fn verify_json_u32_partially_should_fail_when_float() {
                    verify("u32", json!({"age": 42.3}), json!({"age": "{{anyU32}}"}))
                }

                #[should_panic(expected = "Verification failed for stub 'u32'. Expected response body to match '{{anyU32}}' but was 'abcd'")]
                #[test]
                fn verify_json_u32_partially_should_fail_when_string() {
                    verify("u32", json!({"age": "abcd"}), json!({"age": "{{anyU32}}"}))
                }

                #[should_panic(expected = "Verification failed for stub 'u32'. Expected response body to match '{{anyU32}}' but was '-2147483648'")]
                #[test]
                fn verify_json_u32_partially_should_fail_when_negative() {
                    verify("u32", json!({"age": i32::MIN}), json!({"age": "{{anyU32}}"}))
                }

                #[should_panic(expected = "Verification failed for stub 'u32'. Expected response body to match '{{anyU32}}' but was '18446744073709551615'")]
                #[test]
                fn verify_json_u32_partially_should_fail_when_too_large() {
                    verify("u32", json!({"age": u64::MAX}), json!({"age": "{{anyU32}}"}))
                }
            }

            mod i16 {
                use super::*;

                #[test]
                fn should_verify_json_i16_partially() {
                    verify("i16", json!({"age": i16::MAX}), json!({"age": "{{anyI16}}"}));
                    verify("i16", json!({"age": i16::MIN}), json!({"age": "{{anyI16}}"}));
                }

                #[should_panic(expected = "Verification failed for stub 'i16'. Expected response body to match '{{anyI16}}' but was '42.3'")]
                #[test]
                fn verify_json_i16_partially_should_fail_when_float() {
                    verify("i16", json!({"age": 42.3}), json!({"age": "{{anyI16}}"}))
                }

                #[should_panic(expected = "Verification failed for stub 'i16'. Expected response body to match '{{anyI16}}' but was 'abcd'")]
                #[test]
                fn verify_json_i16_partially_should_fail_when_string() {
                    verify("i16", json!({"age": "abcd"}), json!({"age": "{{anyI16}}"}))
                }

                #[should_panic(expected = "Verification failed for stub 'i16'. Expected response body to match '{{anyI16}}' but was '65535'")]
                #[test]
                fn verify_json_i16_partially_should_fail_when_too_large() {
                    verify("i16", json!({"age": u16::MAX}), json!({"age": "{{anyI16}}"}))
                }
            }

            mod u16 {
                use super::*;

                #[test]
                fn should_verify_json_u16_partially() {
                    verify("u16", json!({"age": u16::MAX}), json!({"age": "{{anyU16}}"}));
                    verify("u16", json!({"age": u16::MIN}), json!({"age": "{{anyU16}}"}));
                }

                #[should_panic(expected = "Verification failed for stub 'u16'. Expected response body to match '{{anyU16}}' but was '42.3'")]
                #[test]
                fn verify_json_u16_partially_should_fail_when_float() {
                    verify("u16", json!({"age": 42.3}), json!({"age": "{{anyU16}}"}))
                }

                #[should_panic(expected = "Verification failed for stub 'u16'. Expected response body to match '{{anyU16}}' but was 'abcd'")]
                #[test]
                fn verify_json_u16_partially_should_fail_when_string() {
                    verify("u16", json!({"age": "abcd"}), json!({"age": "{{anyU16}}"}))
                }

                #[should_panic(expected = "Verification failed for stub 'u16'. Expected response body to match '{{anyU16}}' but was '-32768'")]
                #[test]
                fn verify_json_u16_partially_should_fail_when_negative() {
                    verify("u16", json!({"age": i16::MIN}), json!({"age": "{{anyU16}}"}))
                }

                #[should_panic(expected = "Verification failed for stub 'u16'. Expected response body to match '{{anyU16}}' but was '4294967295'")]
                #[test]
                fn verify_json_u16_partially_should_fail_when_too_large() {
                    verify("u16", json!({"age": u32::MAX}), json!({"age": "{{anyU16}}"}))
                }
            }

            mod i8 {
                use super::*;

                #[test]
                fn should_verify_json_i8_partially() {
                    verify("i8", json!({"age": i8::MAX}), json!({"age": "{{anyI8}}"}));
                    verify("i8", json!({"age": i8::MIN}), json!({"age": "{{anyI8}}"}));
                }

                #[should_panic(expected = "Verification failed for stub 'i8'. Expected response body to match '{{anyI8}}' but was '42.3'")]
                #[test]
                fn verify_json_i8_partially_should_fail_when_float() {
                    verify("i8", json!({"age": 42.3}), json!({"age": "{{anyI8}}"}))
                }

                #[should_panic(expected = "Verification failed for stub 'i8'. Expected response body to match '{{anyI8}}' but was 'abcd'")]
                #[test]
                fn verify_json_i8_partially_should_fail_when_string() {
                    verify("i8", json!({"age": "abcd"}), json!({"age": "{{anyI8}}"}))
                }

                #[should_panic(expected = "Verification failed for stub 'i8'. Expected response body to match '{{anyI8}}' but was '255'")]
                #[test]
                fn verify_json_i8_partially_should_fail_when_too_large() {
                    verify("i8", json!({"age": u8::MAX}), json!({"age": "{{anyI8}}"}))
                }
            }

            mod u8 {
                use super::*;

                #[test]
                fn should_verify_json_u8_partially() {
                    verify("u8", json!({"age": u8::MAX}), json!({"age": "{{anyU8}}"}));
                    verify("u8", json!({"age": u8::MIN}), json!({"age": "{{anyU8}}"}));
                }

                #[should_panic(expected = "Verification failed for stub 'u8'. Expected response body to match '{{anyU8}}' but was '42.3'")]
                #[test]
                fn verify_json_u8_partially_should_fail_when_float() {
                    verify("u8", json!({"age": 42.3}), json!({"age": "{{anyU8}}"}))
                }

                #[should_panic(expected = "Verification failed for stub 'u8'. Expected response body to match '{{anyU8}}' but was 'abcd'")]
                #[test]
                fn verify_json_u8_partially_should_fail_when_string() {
                    verify("u8", json!({"age": "abcd"}), json!({"age": "{{anyU8}}"}))
                }

                #[should_panic(expected = "Verification failed for stub 'u8'. Expected response body to match '{{anyU8}}' but was '-128'")]
                #[test]
                fn verify_json_u8_partially_should_fail_when_negative() {
                    verify("u8", json!({"age": i8::MIN}), json!({"age": "{{anyU8}}"}))
                }

                #[should_panic(expected = "Verification failed for stub 'u8'. Expected response body to match '{{anyU8}}' but was '-32768'")]
                #[test]
                fn verify_json_u8_partially_should_fail_when_too_large() {
                    verify("u8", json!({"age": i16::MIN}), json!({"age": "{{anyU8}}"}))
                }
            }
        }

        mod float {
            use super::*;

            #[test]
            fn should_verify_json_float_partially() {
                verify("float", json!({"age": 42.3}), json!({"age": "{{anyFloat}}"}))
            }

            #[should_panic(expected = "Verification failed for stub 'float'. Expected response body to match '{{anyFloat}}' but was 'abcd'")]
            #[test]
            fn verify_json_float_partially_should_fail() {
                verify("float", json!({"age": "abcd"}), json!({"age": "{{anyFloat}}"}))
            }

            #[should_panic(expected = "Verification failed for stub 'float'. Expected response body to match '{{anyFloat}}' but was '42'")]
            #[test]
            fn verify_json_float_partially_should_fail_when_int() {
                verify("float", json!({"age": 42}), json!({"age": "{{anyFloat}}"}))
            }
        }

        mod alpha_numeric {
            use super::*;

            #[test]
            fn should_verify_json_alpha_numeric_partially() {
                verify("alpha-num", json!({"age": "abcd1234ABCD"}), json!({"age": "{{anyAlphaNumeric}}"}));
            }

            #[should_panic(expected = "Verification failed for stub 'alpha-num'. Expected response body to match '{{anyAlphaNumeric}}' but was '!?'")]
            #[test]
            fn verify_json_alpha_numeric_partially_should_fail() {
                verify("alpha-num", json!({"age": "!?"}), json!({"age": "{{anyAlphaNumeric}}"}))
            }
        }
    }

    fn verify(name: &str, actual: Value, expected: Value) {
        verify_with_uri(name, actual, expected, "http://localhost/")
    }

    fn verify_with_uri(name: &str, actual: Value, expected: Value, uri: &str) {
        let stub = stub(&expected);
        let mut req = Request::post(uri);
        req.set_body(actual.clone());
        let mut resp = Response::new(200);
        resp.set_body(actual.clone());
        JsonBodyTemplatingVerifier { actual, expected }
            .verify(&stub, name, &RequestData::from(&mut req), &mut StdResponse(resp));
    }

    fn stub(expected: &Value) -> ResponseStub {
        ResponseStub {
            body: BodyStub { json_body: Some(expected.clone()), ..Default::default() },
            transformers: vec![String::from("response-template")],
            ..Default::default()
        }
    }
}
