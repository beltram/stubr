use std::hash::{Hash, Hasher};

use itertools::Itertools;
use json_value_merge::Merge;
use serde_json::{json, Value};

use crate::{
    gen::contains::StringRndGenerator,
    model::request::{body::BodyPatternStub, RequestStub},
    verify::mapping::jsonpath::JsonGeneratorIterator,
};

use super::super::jsonpath::JsonPathGenerator;

impl From<&RequestStub> for Vec<u8> {
    fn from(stub: &RequestStub) -> Self {
        stub.body_patterns.iter()
            .map(PartialBody::from)
            .find(|it| !it.is_partial())
            .and_then(PartialBody::to_bytes)
            .unwrap_or_else(|| {
                let merged = stub.body_patterns.iter()
                    .map(PartialBody::from)
                    .unique()
                    .fold(Value::default(), |mut acc, it| {
                        if let Some(value) = it.to_partial_value() {
                            acc.merge(value);
                        }
                        acc
                    });
                serde_json::to_vec::<Value>(&merged).unwrap()
            })
    }
}

#[derive(Default, Eq, Clone)]
struct PartialBody {
    path: Option<String>,
    bytes: Option<Vec<u8>>,
    value: Option<Value>,
}

lazy_static! {
    pub static ref EMPTY_JSON_OBJECT: Value = serde_json::json!({});
}

impl PartialBody {
    fn is_partial(&self) -> bool {
        self.path.is_some()
    }

    #[allow(clippy::wrong_self_convention)]
    fn to_bytes(self) -> Option<Vec<u8>> {
        if !self.is_partial() {
            self.bytes.to_owned()
                .or_else(|| self.to_value().as_ref().and_then(|it| serde_json::to_vec::<Value>(it).ok()))
        } else { None }
    }

    #[allow(clippy::wrong_self_convention)]
    fn to_value(self) -> Option<Value> {
        if !self.is_partial() { self.value } else { None }
    }

    fn to_partial_value(&self) -> Option<Value> {
        self.path.as_deref()
            .and_then(|path| JsonPathGenerator(path).next(self.value.clone().unwrap_or_else(|| json!({}))))
    }
}

impl From<&BodyPatternStub> for PartialBody {
    fn from(stub: &BodyPatternStub) -> Self {
        if let Some(binary_equal_to) = stub.binary_equal_to.as_ref() {
            base64::decode(binary_equal_to)
                .unwrap_or_else(|_| panic!("'{}' must be Base64 encoded", binary_equal_to))
                .into()
        } else if let Some(expression) = stub.expression.as_ref() {
            if let Some(equal_to_json) = stub.equal_to_json.as_ref() {
                PartialBody { path: Some(expression.to_string()), value: Some(equal_to_json.to_owned()), ..Default::default() }
            } else if let Some(contains) = stub.contains.as_ref() {
                let value = StringRndGenerator::generate_string_containing(contains.to_string());
                PartialBody { path: Some(expression.to_string()), value: Some(Value::String(value)), ..Default::default() }
            } else { PartialBody::default() }
        } else if let Some(eq) = stub.equal_to_json.as_ref() {
            eq.to_owned().into()
        } else if let Some(json_path) = stub.matches_json_path.as_ref() {
            PartialBody { path: Some(json_path.to_owned()), ..Default::default() }
        } else { PartialBody::default() }
    }
}

impl From<Vec<u8>> for PartialBody {
    fn from(bytes: Vec<u8>) -> Self {
        Self { bytes: Some(bytes), ..Default::default() }
    }
}

impl From<Value> for PartialBody {
    fn from(value: Value) -> Self {
        Self { value: Some(value), ..Default::default() }
    }
}

impl PartialEq for PartialBody {
    fn eq(&self, other: &Self) -> bool {
        self.path.eq(&other.path)
    }
}

impl Hash for PartialBody {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}

#[cfg(test)]
mod verify_body_tests {
    use serde_json::{json, Value};

    use super::*;

    mod equal_to_json {
        use super::*;

        #[test]
        fn equal_to_json_should_generate_strictly_equal() {
            let json = json!({"name": "john", "age": 42});
            let stub = BodyPatternStub { equal_to_json: Some(json.clone()), ..Default::default() };
            assert_eq!(PartialBody::from(&stub).to_value().unwrap(), json);
        }
    }

    mod binary_equal_to {
        use super::*;

        #[test]
        fn binary_equal_to_should_generate_strictly_equal() {
            let stub = BodyPatternStub { binary_equal_to: Some(String::from("AQID")), ..Default::default() };
            assert_eq!(PartialBody::from(&stub).to_bytes().unwrap(), vec![1, 2, 3]);
        }

        #[should_panic(expected = "'!!!' must be Base64 encoded")]
        #[test]
        fn binary_equal_to_should_fail_when_not_base64() {
            let _ = PartialBody::from(&BodyPatternStub { binary_equal_to: Some(String::from("!!!")), ..Default::default() });
        }
    }

    mod expression {
        use super::*;

        #[test]
        fn expression_contains_should_generate_containing() {
            let by_contains = BodyPatternStub {
                expression: Some(String::from("$.name")),
                contains: Some(String::from("a")),
                ..Default::default()
            };
            let stub = RequestStub { body_patterns: vec![by_contains], ..Default::default() };
            let body = serde_json::from_slice::<Value>(&Vec::<u8>::from(&stub)).unwrap();
            let name = body.as_object().unwrap().get("name").unwrap();
            assert!(name.as_str().unwrap().contains('a'));
        }

        #[test]
        fn expression_equal_to_json_should_generate_strictly_equal() {
            let owner = json!({"name": "john", "age": 42});
            let by_eq = BodyPatternStub {
                expression: Some(String::from("$.owner")),
                equal_to_json: Some(owner.clone()),
                ..Default::default()
            };
            let stub = RequestStub { body_patterns: vec![by_eq], ..Default::default() };
            let body = serde_json::from_slice::<Value>(&Vec::<u8>::from(&stub)).unwrap();
            assert_eq!(body, json!({"owner": owner}));
        }
    }

    mod many_expression {
        use super::*;

        #[test]
        fn many_expression_equal_to_json_should_generate_combined() {
            let alice = json!({"name": "alice"});
            let sender = BodyPatternStub {
                expression: Some(String::from("$.sender")),
                equal_to_json: Some(alice.clone()),
                ..Default::default()
            };
            let bob = json!({"name": "bob"});
            let receiver = BodyPatternStub {
                expression: Some(String::from("$.receiver")),
                equal_to_json: Some(bob.clone()),
                ..Default::default()
            };
            let stub = RequestStub { body_patterns: vec![sender, receiver], ..Default::default() };
            let body = serde_json::from_slice::<Value>(&Vec::<u8>::from(&stub)).unwrap();
            assert_eq!(body, json!({"sender": alice, "receiver": bob}));
        }

        #[test]
        fn many_expression_equal_to_json_should_merge_paths() {
            let alice = json!({"name": "alice"});
            let alice_stub = BodyPatternStub {
                expression: Some(String::from("$.person.alice")),
                equal_to_json: Some(alice.clone()),
                ..Default::default()
            };
            let bob = json!({"name": "bob"});
            let bob_stub = BodyPatternStub {
                expression: Some(String::from("$.person.bob")),
                equal_to_json: Some(bob.clone()),
                ..Default::default()
            };
            let stub = RequestStub { body_patterns: vec![alice_stub, bob_stub], ..Default::default() };
            let body = serde_json::from_slice::<Value>(&Vec::<u8>::from(&stub)).unwrap();
            assert_eq!(body, json!({"person": {"alice": alice, "bob": bob}}));
        }

        #[test]
        fn many_expression_equal_to_json_and_contains_should_generate_combined() {
            let alice = json!({"name": "alice"});
            let sender = BodyPatternStub {
                expression: Some(String::from("$.sender")),
                equal_to_json: Some(alice.clone()),
                ..Default::default()
            };
            let receiver = BodyPatternStub {
                expression: Some(String::from("$.receiver")),
                contains: Some(String::from("b")),
                ..Default::default()
            };
            let stub = RequestStub { body_patterns: vec![sender, receiver], ..Default::default() };
            let body = serde_json::from_slice::<Value>(&Vec::<u8>::from(&stub)).unwrap();
            let body = body.as_object().unwrap();
            assert_eq!(body.get("sender").unwrap(), &alice);
            assert!(body.get("receiver").unwrap().as_str().unwrap().contains('b'));
        }

        #[test]
        fn many_contains_should_generate_combined() {
            let sender = BodyPatternStub {
                expression: Some(String::from("$.sender")),
                contains: Some(String::from("s")),
                ..Default::default()
            };
            let receiver = BodyPatternStub {
                expression: Some(String::from("$.receiver")),
                contains: Some(String::from("r")),
                ..Default::default()
            };
            let stub = RequestStub { body_patterns: vec![sender, receiver], ..Default::default() };
            let body = serde_json::from_slice::<Value>(&Vec::<u8>::from(&stub)).unwrap();
            let body = body.as_object().unwrap();
            assert!(body.get("sender").unwrap().as_str().unwrap().contains('s'));
            assert!(body.get("receiver").unwrap().as_str().unwrap().contains('r'));
        }
    }

    mod json_path {
        use super::*;

        #[test]
        fn matches_json_path_should_generate_containing_empty_json() {
            let jsonpath = BodyPatternStub {
                matches_json_path: Some(String::from("$.name")),
                ..Default::default()
            };
            let stub = RequestStub { body_patterns: vec![jsonpath], ..Default::default() };
            let body = serde_json::from_slice::<Value>(&Vec::<u8>::from(&stub)).unwrap();
            assert_eq!(body, json!({"name": {}}));
        }

        #[test]
        fn matches_json_path_and_expression_should_generate_valid_json() {
            let owner = json!({"name": "john", "age": 42});
            let by_jsonpath = BodyPatternStub {
                matches_json_path: Some(String::from("$.other")),
                ..Default::default()
            };
            let by_eq = BodyPatternStub {
                expression: Some(String::from("$.owner")),
                equal_to_json: Some(owner.clone()),
                ..Default::default()
            };
            let stub = RequestStub { body_patterns: vec![by_jsonpath, by_eq], ..Default::default() };
            let body = serde_json::from_slice::<Value>(&Vec::<u8>::from(&stub)).unwrap();
            assert_eq!(body, json!({"other": {}, "owner": owner}));
        }
    }

    mod json_path_filtering {
        use super::*;

        mod eq {
            use super::*;

            #[test]
            fn matches_json_path_eq_should_generate_containing_filters() {
                let jsonpath_alice = BodyPatternStub {
                    matches_json_path: Some(String::from("$.users[?(@.name == 'alice')]")),
                    ..Default::default()
                };
                let stub = RequestStub { body_patterns: vec![jsonpath_alice], ..Default::default() };
                let body = serde_json::from_slice::<Value>(&Vec::<u8>::from(&stub)).unwrap();
                assert_eq!(body, json!({"users": [{"name": "alice"}]}));
            }

            #[test]
            fn matches_many_json_path_eq_should_generate_containing_filters() {
                let jsonpath_alice = BodyPatternStub {
                    matches_json_path: Some(String::from("$.users[?(@.name == 'alice')]")),
                    ..Default::default()
                };
                let jsonpath_bob = BodyPatternStub {
                    matches_json_path: Some(String::from("$.users[?(@.name == 'bob')]")),
                    ..Default::default()
                };
                let stub = RequestStub { body_patterns: vec![jsonpath_alice, jsonpath_bob], ..Default::default() };
                let body = serde_json::from_slice::<Value>(&Vec::<u8>::from(&stub)).unwrap();
                assert_eq!(body, json!({"users": [{"name": "alice"}, {"name": "bob"}]}));
            }
        }
    }

    mod precedence {
        use super::*;

        #[test]
        fn binary_equal_to_should_have_precedence_over_equal_to_json() {
            let priority = BodyPatternStub { binary_equal_to: Some(String::from("AQID")), ..Default::default() };
            let other = BodyPatternStub { equal_to_json: Some(json!({"name": "jdoe"})), ..Default::default() };
            let stub = RequestStub { body_patterns: vec![priority, other], ..Default::default() };
            assert_eq!(Vec::<u8>::from(&stub).to_vec(), vec![1, 2, 3]);
        }

        #[test]
        fn binary_equal_to_should_have_precedence_over_expression() {
            let priority = BodyPatternStub { binary_equal_to: Some(String::from("AQID")), ..Default::default() };
            let other = BodyPatternStub { expression: Some(String::from("$.owner")), equal_to_json: Some(json!({"name": "jdoe"})), ..Default::default() };
            let stub = RequestStub { body_patterns: vec![priority, other], ..Default::default() };
            assert_eq!(Vec::<u8>::from(&stub).to_vec(), vec![1, 2, 3]);
        }

        #[test]
        fn equal_to_json_should_have_precedence_over_expression() {
            let jdoe = json!({"name": "jdoe"});
            let priority = BodyPatternStub { equal_to_json: Some(jdoe.clone()), ..Default::default() };
            let other = BodyPatternStub { expression: Some(String::from("$.owner")), equal_to_json: Some(jdoe.clone()), ..Default::default() };
            let stub = RequestStub { body_patterns: vec![priority, other], ..Default::default() };
            let body = serde_json::from_slice::<Value>(&Vec::<u8>::from(&stub)).unwrap();
            assert_eq!(body, jdoe);
        }

        #[test]
        fn expression_equal_to_json_should_have_precedence_over_expression_contains() {
            let jdoe = json!({"name": "jdoe"});
            let priority = BodyPatternStub { expression: Some(String::from("$.owner")), equal_to_json: Some(jdoe.clone()), ..Default::default() };
            let other = BodyPatternStub { expression: Some(String::from("$.owner")), contains: Some(String::from("a")), ..Default::default() };
            let stub = RequestStub { body_patterns: vec![priority, other], ..Default::default() };
            let body = serde_json::from_slice::<Value>(&Vec::<u8>::from(&stub)).unwrap();
            assert_eq!(body, json!({"owner": jdoe}));
        }

        #[test]
        fn expression_should_have_precedence_over_matches_json_path() {
            let jdoe = json!({"name": "jdoe"});
            let priority = BodyPatternStub { expression: Some(String::from("$.owner")), equal_to_json: Some(jdoe.clone()), ..Default::default() };
            let other = BodyPatternStub { matches_json_path: Some(String::from("$.owner")), ..Default::default() };
            let stub = RequestStub { body_patterns: vec![priority, other], ..Default::default() };
            let body = serde_json::from_slice::<Value>(&Vec::<u8>::from(&stub)).unwrap();
            assert_eq!(body, json!({"owner": jdoe}));
        }
    }
}