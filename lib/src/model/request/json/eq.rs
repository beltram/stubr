use serde_json::Value;

use super::JsonMatcher;

pub struct JsonExactMatcher<'a>(pub &'a Value);

impl <'a> JsonMatcher<'a> for JsonExactMatcher<'a> {
    fn matches(&self, json: &'a Value) -> bool {
        self.0 == json
    }
}