use jsonpath_lib::Compiled;
use serde_json::Value;

use super::JsonMatcher;

pub struct JsonPathMatcher<'a>(pub &'a Compiled);

impl<'a> JsonMatcher<'a> for JsonPathMatcher<'a> {
    fn matches(&self, json: &'a Value) -> bool {
        self.0.select(json).ok().filter(|it| !it.is_empty()).is_some()
    }
}
