use jsonpath_plus::JsonPath;
use serde_json::Value;

mod segment;
mod dot;
mod bracket;
mod filter;
mod lit;

pub trait JsonGeneratorIterator {
    fn next(self, acc_json: Value) -> Option<Value>;
}

#[allow(dead_code)]
pub struct JsonPathGenerator<'a>(pub &'a str);

impl JsonGeneratorIterator for JsonPathGenerator<'_> {
    fn next(self, acc_json: Value) -> Option<Value> {
        JsonPath::compile(self.0).ok()
            .and_then(|p: JsonPath| segment::SegmentsGenerator(p.segments()).next(acc_json))
    }
}
