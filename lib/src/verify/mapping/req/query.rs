use std::convert::TryInto;

use crate::model::request::{matcher::RequestMatcherStub, query::HttpQueryParamsStub};

impl From<&HttpQueryParamsStub> for Vec<(String, String)> {
    fn from(queries: &HttpQueryParamsStub) -> Self {
        queries.get_queries()
            .map(|iter| {
                iter.filter_map(|RequestMatcherStub { key, value }| {
                    Some(key.to_string())
                        .zip(value.as_ref().and_then(|it| it.try_into().ok()))
                }).collect()
            }).unwrap_or_default()
    }
}

#[cfg(test)]
mod verify_query_tests {
    use std::{iter::FromIterator, str::FromStr};

    use regex::Regex;
    use serde_json::{Map, Value};

    use crate::model::request::{matcher::MatcherValueStub, query::HttpQueryParamsStub};

    #[test]
    fn equal_to_should_generate_exact() {
        let matcher = MatcherValueStub { equal_to: Some(Value::String(String::from("bcd"))), ..Default::default() };
        let matcher = serde_json::to_value(matcher).unwrap();
        let query_parameters = vec![(String::from("a"), matcher)];
        let queries = HttpQueryParamsStub { query_parameters: Some(Map::from_iter(query_parameters)) };
        let queries = Vec::<(String, String)>::from(&queries);
        assert_eq!(queries.len(), 1);
        assert_eq!(queries.get(0).unwrap().0, "a");
        assert_eq!(queries.get(0).unwrap().1, "bcd");
    }

    #[test]
    fn contains_to_should_generate_containing() {
        let matcher = MatcherValueStub { contains: Some(String::from("b")), ..Default::default() };
        let matcher = serde_json::to_value(matcher).unwrap();
        let query_parameters = vec![(String::from("a"), matcher)];
        let queries = HttpQueryParamsStub { query_parameters: Some(Map::from_iter(query_parameters)) };
        let queries = Vec::<(String, String)>::from(&queries);
        assert_eq!(queries.len(), 1);
        assert_eq!(queries.get(0).unwrap().0, "a");
        assert!(queries.get(0).unwrap().1.contains('b'));
    }

    #[test]
    fn matches_to_should_generate_matching_regex() {
        let regex = "[a-z]{4}";
        let matcher = MatcherValueStub { matches: Some(Value::String(String::from(regex))), ..Default::default() };
        let matcher = serde_json::to_value(matcher).unwrap();
        let query_parameters = vec![(String::from("a"), matcher)];
        let queries = HttpQueryParamsStub { query_parameters: Some(Map::from_iter(query_parameters)) };
        let queries = Vec::<(String, String)>::from(&queries);
        assert_eq!(queries.len(), 1);
        assert_eq!(queries.get(0).unwrap().0, "a");
        let regex = Regex::from_str(regex).unwrap();
        assert!(regex.is_match(&queries.get(0).unwrap().1));
    }
}