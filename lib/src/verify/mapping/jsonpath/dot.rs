use jsonpath_plus::ast::RawSelector;
use serde_json::{json, Value};

use super::JsonGeneratorIterator;

pub struct DotGenerator<'a>(pub &'a RawSelector);

impl JsonGeneratorIterator for DotGenerator<'_> {
    fn next(self, acc_json: Value) -> Option<Value> {
        match self.0 {
            RawSelector::Name(name) => Some(json!({name.as_str(): acc_json})),
            _ => None
        }
    }
}

#[cfg(test)]
mod jsonpath_generator_dot_tests {
    use serde_json::json;

    use super::{*, super::JsonPathGenerator};

    mod name {
        use super::*;

        #[test]
        fn single_dot_should_generate() {
            let doe = json!({"name": "doe"});
            let value = JsonPathGenerator("$.a").next(doe.clone()).unwrap();
            assert_eq!(value, json!({"a": doe}));
        }

        #[test]
        fn many_dots_generate() {
            let doe = json!({"name": "doe"});
            let value = JsonPathGenerator("$.a.b.c").next(doe.clone()).unwrap();
            assert_eq!(value, json!({"a": { "b": { "c": doe } }}));
        }
    }
}