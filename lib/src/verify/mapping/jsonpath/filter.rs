use jsonpath_plus::ast::{BinOp, FilterExpr};
use serde_json::Value;

use super::{
    JsonGeneratorIterator,
    lit::LitGenerator,
    segment::SegmentsGenerator,
};

pub struct FilterGenerator<'a>(pub &'a FilterExpr, pub Option<&'a BinOp>);

impl JsonGeneratorIterator for FilterGenerator<'_> {
    fn next(self, acc_json: Value) -> Option<Value> {
        match self.0 {
            FilterExpr::Binary(left, operator, right) => {
                Self(right, Some(operator)).next(acc_json)
                    .and_then(|acc| Self(left, None).next(acc))
            }
            FilterExpr::Path(path) => SegmentsGenerator(path.segments()).next(acc_json),
            FilterExpr::Lit(expr) => LitGenerator(expr, self.1).next(acc_json),
            _ => None
        }
    }
}


#[cfg(test)]
mod jsonpath_generator_filter_tests {
    use serde_json::json;

    use super::{*, super::JsonPathGenerator};

    mod binary {
        use super::*;

        #[test]
        fn filter_should_generate() {
            let value = JsonPathGenerator("$.users[?(@.a == 'b')]").next(json!({})).unwrap();
            assert_eq!(value, json!({"users": [{"a": "b"}]}));
        }

        #[test]
        fn filter_many_dot_path_should_generate() {
            let value = JsonPathGenerator("$.users[?(@.a.b == 'c')]").next(json!({})).unwrap();
            assert_eq!(value, json!({"users": [{"a": {"b": "c"}}]}));
        }
    }
}
