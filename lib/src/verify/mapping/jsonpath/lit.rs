use jsonpath_plus::ast::{BinOp, BoolLit, ExprLit, IntLit, StringLit};
use serde_json::{json, Value};

use super::JsonGeneratorIterator;

pub struct LitGenerator<'a>(pub &'a ExprLit, pub Option<&'a BinOp>);

impl LitGenerator<'_> {
    fn int(&self, raw: &IntLit) -> Option<Value> {
        self.1.and_then(|op| match op {
            BinOp::Eq(_) => Some(json!(raw.as_int())),
            _ => None
        })
    }

    fn str(&self, raw: &StringLit) -> Option<Value> {
        self.1.and_then(|op| match op {
            BinOp::Eq(_) => Some(json!(raw.as_str())),
            _ => None
        })
    }

    fn boolean(&self, raw: &BoolLit) -> Option<Value> {
        self.1.and_then(|op| match op {
            BinOp::Eq(_) => Some(json!(raw.as_bool())),
            _ => None
        })
    }

    fn null(&self) -> Option<Value> {
        self.1.and_then(|op| match op {
            BinOp::Eq(_) => Some(json!(null)),
            _ => None
        })
    }
}

impl JsonGeneratorIterator for LitGenerator<'_> {
    fn next(self, _: Value) -> Option<Value> {
        match self.0 {
            ExprLit::Int(i) => self.int(i),
            ExprLit::String(s) => self.str(s),
            ExprLit::Bool(b) => self.boolean(b),
            ExprLit::Null(_) => self.null(),
            _ => None
        }
    }
}

#[cfg(test)]
mod jsonpath_generator_lit {
    use super::{*, super::JsonPathGenerator};

    mod string {
        use super::*;

        #[test]
        fn eq_should_generate() {
            let value = JsonPathGenerator("$.users[?(@.a == 'b')]").next(json!({})).unwrap();
            assert_eq!(value, json!({"users": [{"a": "b"}]}));
        }
    }

    mod int {
        use super::*;

        #[test]
        fn eq_should_generate() {
            let value = JsonPathGenerator("$.users[?(@.a == 42)]").next(json!({})).unwrap();
            assert_eq!(value, json!({"users": [{"a": 42}]}));
        }
    }

    mod bool {
        use super::*;

        #[test]
        fn eq_should_generate() {
            let value = JsonPathGenerator("$.users[?(@.a == true)]").next(json!({})).unwrap();
            assert_eq!(value, json!({"users": [{"a": true}]}));
        }
    }

    mod boolean {
        use super::*;

        #[test]
        fn eq_should_generate() {
            let value = JsonPathGenerator("$.users[?(@.a == null)]").next(json!({})).unwrap();
            assert_eq!(value, json!({"users": [{"a": null}]}));
        }
    }
}