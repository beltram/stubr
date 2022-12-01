use serde_json::Value;

use super::JsonMatcher;

pub struct JsonPathContainsMatcher<'a>(pub &'a String, pub &'a String);

impl<'a> JsonMatcher<'a> for JsonPathContainsMatcher<'a> {
    fn matches(&self, json: &'a Value) -> bool {
        jsonpath_lib::select(json, self.0)
            .ok()
            .filter(|matched| !matched.is_empty())
            .filter(|matched| {
                matched
                    .iter()
                    .all(|it| it.as_str().map(|c| c.contains(self.1.as_str())).unwrap_or_default())
            })
            .is_some()
    }
}
