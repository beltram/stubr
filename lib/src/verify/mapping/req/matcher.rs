use std::str::FromStr;

use anyhow::Error;
use itertools::Itertools;
use serde_json::Value;

use crate::{
    gen::{contains::StringRndGenerator, regex::RegexRndGenerator},
    model::request::matcher::MatcherValueStub,
};

struct MatcherValueStubMapper;

impl MatcherValueStubMapper {
    fn map_equal_to(value: &Value, case_insensitive: bool) -> Option<String> {
        value.as_str()
            .map(&str::to_string)
            .or_else(|| value.as_i64().map(|it| it.to_string()))
            .or_else(|| value.as_f64().map(|it| it.to_string()))
            .or_else(|| value.as_bool().map(|it| it.to_string()))
            .or_else(|| value.as_null().map(|_| String::from("null")))
            .map(|it| if case_insensitive { Self::map_case_insensitive(it) } else { it })
    }

    fn map_case_insensitive(value: String) -> String {
        value.char_indices()
            .map(|(i, c)| if i % 2 == 0 { c.to_uppercase().to_string() } else { c.to_lowercase().to_string() })
            .join("")
    }

    fn map_contains(value: &str) -> Option<String> {
        i64::from_str(value).ok()
            .map(StringRndGenerator::generate_number_containing)
            .or_else(|| Some(StringRndGenerator::generate_string_containing(value.to_string())))
    }

    fn map_matches(value: &str) -> Option<String> {
        RegexRndGenerator::try_from(value).ok()
            .and_then(|g| g.try_generate().ok())
    }
}

impl TryFrom<&MatcherValueStub> for String {
    type Error = Error;

    fn try_from(matcher: &MatcherValueStub) -> anyhow::Result<Self> {
        if let Some(equal_to) = matcher.equal_to.as_ref() {
            let case_insensitive = matcher.case_insensitive.unwrap_or_default();
            MatcherValueStubMapper::map_equal_to(equal_to, case_insensitive)
                .ok_or_else(|| Error::msg("Invalid 'equal_to'"))
        } else if let Some(contains) = matcher.contains.as_ref() {
            MatcherValueStubMapper::map_contains(contains)
                .ok_or_else(|| Error::msg("Invalid 'contains'"))
        } else if let Some(matches) = matcher.matches.as_ref().and_then(Value::as_str) {
            MatcherValueStubMapper::map_matches(matches)
                .ok_or_else(|| Error::msg("Invalid 'matches'"))
        } else {
            Err(Error::msg("No matcher defined"))
        }
    }
}

#[cfg(test)]
mod verify_matcher_tests {
    use serde_json::json;

    use super::*;

    mod equal_to {
        use super::*;

        #[test]
        fn equal_to_should_map_string_exactly() {
            let matcher = MatcherValueStub { equal_to: Some(json!("exact")), ..Default::default() };
            assert_eq!(String::try_from(&matcher).unwrap(), String::from("exact"));
        }

        #[test]
        fn equal_to_should_map_number_exactly() {
            let matcher = MatcherValueStub { equal_to: Some(json!(42)), ..Default::default() };
            assert_eq!(String::try_from(&matcher).unwrap(), String::from("42"));
        }

        #[test]
        fn equal_to_should_map_float_exactly() {
            let matcher = MatcherValueStub { equal_to: Some(json!(1.6)), ..Default::default() };
            assert_eq!(String::try_from(&matcher).unwrap(), String::from("1.6"));
        }

        #[test]
        fn equal_to_should_map_boolean_exactly() {
            let matcher = MatcherValueStub { equal_to: Some(json!(true)), ..Default::default() };
            assert_eq!(String::try_from(&matcher).unwrap(), String::from("true"));
        }

        #[test]
        fn equal_to_should_map_null_exactly() {
            let matcher = MatcherValueStub { equal_to: Some(json!(null)), ..Default::default() };
            assert_eq!(String::try_from(&matcher).unwrap(), String::from("null"));
        }

        #[test]
        fn equal_to_case_insensitive_should_map_with_some_uppercase() {
            let matcher = MatcherValueStub { equal_to: Some(json!("exact")), case_insensitive: Some(true), ..Default::default() };
            assert_eq!(String::try_from(&matcher).unwrap(), String::from("ExAcT"));
        }

        #[test]
        fn equal_to_case_sensitive_should_preserve_case() {
            let matcher = MatcherValueStub { equal_to: Some(json!("exact")), case_insensitive: Some(false), ..Default::default() };
            assert_eq!(String::try_from(&matcher).unwrap(), String::from("exact"));
            let matcher = MatcherValueStub { equal_to: Some(json!("EXACT")), case_insensitive: Some(false), ..Default::default() };
            assert_eq!(String::try_from(&matcher).unwrap(), String::from("EXACT"));
        }
    }

    mod contains {
        use super::*;

        #[test]
        fn should_generate_param_containing_string() {
            let matcher = MatcherValueStub { contains: Some(String::from("alpha")), ..Default::default() };
            assert!(String::try_from(&matcher).unwrap().contains("alpha"));
        }

        #[test]
        fn should_generate_param_starting_with_number() {
            let matcher = MatcherValueStub { contains: Some(String::from("42")), ..Default::default() };
            assert!(String::try_from(&matcher).unwrap().starts_with("42"));
        }
    }

    mod matches {
        use super::*;

        #[test]
        fn should_generate_param_matching_string() {
            let regex = String::from("[a-z]{4}");
            let matcher = MatcherValueStub { matches: Some(Value::String(regex.clone())), ..Default::default() };
            let regex = regex::Regex::from_str(&regex).unwrap();
            assert!(regex.is_match(&String::try_from(&matcher).unwrap()));
        }
    }
}