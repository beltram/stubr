use jsonpath_plus::ast::BracketSelector;
use serde_json::{json, Value};

use filter::FilterGenerator;

use super::{filter, JsonGeneratorIterator};

pub struct BracketGenerator<'a>(pub &'a BracketSelector);

impl JsonGeneratorIterator for BracketGenerator<'_> {
    fn next(self, acc_json: Value) -> Option<Value> {
        match self.0 {
            BracketSelector::Filter(filter) => {
                FilterGenerator(filter.expression(), None).next(acc_json)
            }
            _ => None
        }.map(|j| json!([j])) // wrap in an array since we are in brackets []
    }
}