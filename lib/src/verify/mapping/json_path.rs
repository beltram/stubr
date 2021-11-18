use std::iter::FromIterator;

use jsonpath_plus::{ast::{RawSelector, Segment}, JsonPath};
use serde_json::{Map, Value};

#[allow(dead_code)]
pub struct JsonPathGenerator;

impl JsonPathGenerator {
    #[allow(dead_code)]
    pub fn generate_path(path: &str, value: Value) -> Value {
        JsonPath::compile(path).ok()
            .and_then(|p: JsonPath| {
                p.segments().iter()
                    .rev()
                    .fold(None, |mut acc: Option<Value>, segment| {
                        match segment {
                            Segment::Dot(_, selector) => {
                                match selector {
                                    RawSelector::Name(name) => {
                                        let local_path = name.as_str().to_string();
                                        let val = if let Some(m) = acc { m } else { value.clone() };
                                        acc = Some(Value::Object(Map::from_iter(vec![(local_path, val)])));
                                    }
                                    _ => todo!()
                                }
                            }
                            _ => todo!()
                        }
                        acc
                    })
            }).expect(&format!("Failed generating a json value at path '{}'", path))
    }
}

#[cfg(test)]
mod json_path_generator_tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn should_generate_simple() {
        let value = JsonPathGenerator::generate_path("$.a", json!({"name": "doe"}));
        assert_eq!(value, json!({"a": {"name": "doe"}}));
    }

    #[test]
    fn should_generate_nested_path() {
        let value = JsonPathGenerator::generate_path("$.a.b.c", json!({"name": "doe"}));
        assert_eq!(value, json!({"a": { "b": { "c": {"name": "doe"} } }}));
    }
}