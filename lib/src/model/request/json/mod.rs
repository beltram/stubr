use serde_json::Value;

pub mod eq;
pub mod json_path;
pub mod json_path_contains;
pub mod json_path_eq;

pub trait JsonMatcher<'a> {
    fn matches(&self, json: &'a Value) -> bool;
}
