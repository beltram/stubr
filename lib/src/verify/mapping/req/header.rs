use std::convert::TryInto;

use crate::model::request::{headers::HttpReqHeadersStub, matcher::RequestMatcherStub};

impl From<&HttpReqHeadersStub> for Vec<(String, String)> {
    fn from(headers: &HttpReqHeadersStub) -> Self {
        headers.get_headers().iter()
            .filter_map(|RequestMatcherStub { key, value }| {
                Some(key.to_string())
                    .zip(value.as_ref().and_then(|it| it.try_into().ok()))
            })
            .collect()
    }
}

#[cfg(test)]
mod verify_header_tests {
    use std::{iter::FromIterator, str::FromStr};

    use regex::Regex;
    use serde_json::{Map, Value};

    use crate::model::request::matcher::MatcherValueStub;

    use super::*;

    #[test]
    fn equal_to_should_generate_exact() {
        let matcher = MatcherValueStub { equal_to: Some(Value::String(String::from("bcd"))), ..Default::default() };
        let matcher = serde_json::to_value(matcher).unwrap();
        let headers = vec![(String::from("a"), matcher)];
        let headers = HttpReqHeadersStub { headers: Some(Map::from_iter(headers)) };
        let headers = Vec::<(String, String)>::from(&headers);
        assert_eq!(headers.len(), 1);
        assert_eq!(headers.get(0).unwrap().0, "a");
        assert_eq!(headers.get(0).unwrap().1, "bcd");
    }

    #[test]
    fn contains_to_should_generate_containing() {
        let matcher = MatcherValueStub { contains: Some(String::from("b")), ..Default::default() };
        let matcher = serde_json::to_value(matcher).unwrap();
        let headers = vec![(String::from("a"), matcher)];
        let headers = HttpReqHeadersStub { headers: Some(Map::from_iter(headers)) };
        let headers = Vec::<(String, String)>::from(&headers);
        assert_eq!(headers.len(), 1);
        assert_eq!(headers.get(0).unwrap().0, "a");
        assert!(headers.get(0).unwrap().1.contains("b"));
    }

    #[test]
    fn matches_to_should_generate_matching_regex() {
        let regex = "[a-z]{4}";
        let matcher = MatcherValueStub { matches: Some(Value::String(String::from(regex.clone()))), ..Default::default() };
        let matcher = serde_json::to_value(matcher).unwrap();
        let headers = vec![(String::from("a"), matcher)];
        let headers = HttpReqHeadersStub { headers: Some(Map::from_iter(headers)) };
        let headers = Vec::<(String, String)>::from(&headers);
        assert_eq!(headers.len(), 1);
        assert_eq!(headers.get(0).unwrap().0, "a");
        let regex = Regex::from_str(&regex).unwrap();
        assert!(regex.is_match(&headers.get(0).unwrap().1));
    }
}