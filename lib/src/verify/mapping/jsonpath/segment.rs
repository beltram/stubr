use jsonpath_plus::ast::Segment;
use serde_json::Value;

use super::{bracket::BracketGenerator, dot::DotGenerator, JsonGeneratorIterator};

pub struct SegmentsGenerator<'a>(pub &'a [Segment]);

impl JsonGeneratorIterator for SegmentsGenerator<'_> {
    fn next(self, acc_json: Value) -> Option<Value> {
        self.0.iter().rev().fold(None, |acc: Option<Value>, segment| {
            SegmentGenerator(segment).next(acc.unwrap_or_else(|| acc_json.to_owned()))
        })
    }
}

pub struct SegmentGenerator<'a>(pub &'a Segment);

impl JsonGeneratorIterator for SegmentGenerator<'_> {
    fn next(self, acc_json: Value) -> Option<Value> {
        match self.0 {
            Segment::Dot(_, selector) => DotGenerator(selector).next(acc_json),
            Segment::Bracket(_, selector) => BracketGenerator(selector).next(acc_json),
            _ => None,
        }
    }
}
