use serde_json::Value;

use super::JsonMatcher;

pub struct JsonPathEqMatcher<'a>(pub &'a String, pub &'a Value);

impl<'a> JsonMatcher<'a> for JsonPathEqMatcher<'a> {
    fn matches(&self, json: &'a Value) -> bool {
        jsonpath_lib::select(json, &self.0).ok()
            .filter(|matched| !matched.is_empty())
            .filter(|matched| matched.iter().all(|&it| it == self.1))
            .is_some()
    }
}