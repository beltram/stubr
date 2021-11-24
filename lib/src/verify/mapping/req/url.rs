use std::str::FromStr;

use http_types::Url;
use regex::Regex;

use crate::model::request::RequestStub;

use super::super::regex::RegexStub;

struct UrlStubMapper;

impl UrlStubMapper {
    fn url_from_matcher(stub: &RequestStub) -> String {
        Self::url_matcher(stub)
            .map(|(url, is_pattern)| {
                if is_pattern {
                    RegexStub(Regex::from_str(url).unwrap()).into()
                } else { url.to_string() }
            })
            .unwrap_or_default()
    }

    fn url_matcher(stub: &RequestStub) -> Option<(&String, bool)> {
        stub.url.url.as_ref().map(|it| (it, false))
            .or_else(|| stub.url.url_path.as_ref().map(|it| (it, false)))
            .or_else(|| stub.url.url_pattern.as_ref().map(|it| (it, true)))
            .or_else(|| stub.url.url_path_pattern.as_ref().map(|it| (it, true)))
    }
}

impl From<&RequestStub> for Url {
    fn from(stub: &RequestStub) -> Self {
        const BASE_URL: &str = "http://localhost/";
        let url = Self::parse(BASE_URL).unwrap();
        let mut url = url.join(&UrlStubMapper::url_from_matcher(stub)).unwrap();
        for (k, v) in Vec::<(String, String)>::from(&stub.queries).into_iter() {
            url.query_pairs_mut().append_pair(k.as_str(), v.as_str());
        }
        url
    }
}

#[cfg(test)]
mod verify_url_tests {
    use std::borrow::Cow;

    use crate::model::request::url::HttpUrlStub;

    use super::*;

    mod url {
        use super::*;

        #[test]
        fn should_map_exact_path() {
            let stub: RequestStub = HttpUrlStub { url: Some(String::from("/api/exact")), ..Default::default() }.into();
            assert_eq!(Url::from(&stub).path(), "/api/exact")
        }

        #[test]
        fn should_map_exact_path_and_query() {
            let stub: RequestStub = HttpUrlStub { url: Some(String::from("/api/exact?a=b")), ..Default::default() }.into();
            let url = Url::from(&stub);
            assert_eq!(url.path(), "/api/exact");
            let mut queries = url.query_pairs();
            assert_eq!(queries.count(), 1);
            assert_eq!(queries.next().unwrap(), (Cow::Borrowed("a"), Cow::Borrowed("b")));
        }

        #[test]
        fn should_map_exact_path_and_queries() {
            let stub: RequestStub = HttpUrlStub { url: Some(String::from("/api/exact?a=b&c=d&e=f")), ..Default::default() }.into();
            let url = Url::from(&stub);
            assert_eq!(url.path(), "/api/exact");
            let mut queries = url.query_pairs();
            assert_eq!(queries.count(), 3);
            assert_eq!(queries.next().unwrap(), (Cow::Borrowed("a"), Cow::Borrowed("b")));
            assert_eq!(queries.next().unwrap(), (Cow::Borrowed("c"), Cow::Borrowed("d")));
            assert_eq!(queries.next().unwrap(), (Cow::Borrowed("e"), Cow::Borrowed("f")));
        }

        #[test]
        fn should_not_fail_when_path_missing() {
            let stub: RequestStub = HttpUrlStub { url: None, ..Default::default() }.into();
            assert_eq!(Url::from(&stub).path(), "/")
        }
    }

    mod url_path {
        use super::*;

        #[test]
        fn should_map_exact_path() {
            let stub: RequestStub = HttpUrlStub { url_path: Some(String::from("/api/exact")), ..Default::default() }.into();
            assert_eq!(Url::from(&stub).path(), "/api/exact")
        }

        #[test]
        fn should_not_fail_when_url_path_missing() {
            let stub: RequestStub = HttpUrlStub { url_path: None, ..Default::default() }.into();
            assert_eq!(Url::from(&stub).path(), "/")
        }
    }

    mod url_path_pattern {
        use std::str::FromStr;

        use regex::Regex;

        use super::*;

        #[test]
        fn should_map_url_path_pattern() {
            let regex = "/api/regex/([a-z]{4})";
            let stub: RequestStub = HttpUrlStub { url_path_pattern: Some(String::from(regex)), ..Default::default() }.into();
            let url = Url::from(&stub);
            assert!(url.path().starts_with("/api/regex/"));
            let regex = Regex::from_str(regex).unwrap();
            assert!(regex.is_match(url.path()));
        }

        #[test]
        fn should_not_fail_when_url_path_pattern_missing() {
            let stub: RequestStub = HttpUrlStub { url_path_pattern: None, ..Default::default() }.into();
            assert_eq!(Url::from(&stub).path(), "/")
        }
    }

    mod url_pattern {
        use std::str::FromStr;

        use regex::Regex;

        use super::*;

        #[test]
        fn should_map_url_pattern() {
            let (path_regex, query_regex) = ("([a-z]{4})", "([a-z]{4})");
            let regex = format!("/api/regex/{}\\?a={}", path_regex, query_regex);
            let stub: RequestStub = HttpUrlStub { url_pattern: Some(String::from(regex)), ..Default::default() }.into();
            let url = Url::from(&stub);
            assert!(url.path().starts_with("/api/regex/"));
            let regex = Regex::from_str(path_regex).unwrap();
            assert!(regex.is_match(url.as_str()));
            let mut queries = url.query_pairs();
            assert_eq!(queries.count(), 1);
            let (k,v) = queries.next().unwrap();
            assert_eq!(k, Cow::Borrowed("a"));
            assert!(Regex::from_str(query_regex).unwrap().is_match(&v.to_string()));
        }

        #[test]
        fn should_not_fail_when_url_pattern_missing() {
            let stub: RequestStub = HttpUrlStub { url_pattern: None, ..Default::default() }.into();
            assert_eq!(Url::from(&stub).path(), "/")
        }
    }

    mod precedence {
        use super::*;

        #[test]
        fn url_should_have_precedence_over_url_path() {
            let stub: RequestStub = HttpUrlStub {
                url: Some(String::from("/url")),
                url_path: Some(String::from("/url-path")),
                ..Default::default()
            }.into();
            assert_eq!(UrlStubMapper::url_matcher(&stub), Some((&String::from("/url"), false)))
        }

        #[test]
        fn url_should_have_precedence_over_url_pattern() {
            let stub: RequestStub = HttpUrlStub {
                url: Some(String::from("/url")),
                url_pattern: Some(String::from("/url-pattern")),
                ..Default::default()
            }.into();
            assert_eq!(UrlStubMapper::url_matcher(&stub), Some((&String::from("/url"), false)))
        }

        #[test]
        fn url_should_have_precedence_over_url_path_pattern() {
            let stub: RequestStub = HttpUrlStub {
                url: Some(String::from("/url")),
                url_path_pattern: Some(String::from("/url-path-pattern")),
                ..Default::default()
            }.into();
            assert_eq!(UrlStubMapper::url_matcher(&stub), Some((&String::from("/url"), false)))
        }

        #[test]
        fn url_path_should_have_precedence_over_url_pattern() {
            let stub: RequestStub = HttpUrlStub {
                url_path: Some(String::from("/url-path")),
                url_pattern: Some(String::from("/url-pattern")),
                ..Default::default()
            }.into();
            assert_eq!(UrlStubMapper::url_matcher(&stub), Some((&String::from("/url-path"), false)))
        }

        #[test]
        fn url_path_should_have_precedence_over_url_path_pattern() {
            let stub: RequestStub = HttpUrlStub {
                url_path: Some(String::from("/url-path")),
                url_path_pattern: Some(String::from("/url-path-pattern")),
                ..Default::default()
            }.into();
            assert_eq!(UrlStubMapper::url_matcher(&stub), Some((&String::from("/url-path"), false)))
        }

        #[test]
        fn url_pattern_should_have_precedence_over_url_path_pattern() {
            let stub: RequestStub = HttpUrlStub {
                url_pattern: Some(String::from("/url-pattern")),
                url_path_pattern: Some(String::from("/url-path-pattern")),
                ..Default::default()
            }.into();
            assert_eq!(UrlStubMapper::url_matcher(&stub), Some((&String::from("/url-pattern"), true)))
        }
    }

    mod query {
        use std::iter::FromIterator;

        use serde_json::{Map, Value};

        use crate::model::request::{matcher::MatcherValueStub, query::HttpQueryParamsStub};

        use super::*;

        #[test]
        fn should_map_url_with_queries() {
            let url = HttpUrlStub { url: Some(String::from("/api/exact?a=b")), ..Default::default() };
            let matcher_c = MatcherValueStub { equal_to: Some(Value::String(String::from("d"))), ..Default::default() };
            let matcher_c = serde_json::to_value(matcher_c).unwrap();
            let matcher_e = MatcherValueStub { equal_to: Some(Value::String(String::from("f"))), ..Default::default() };
            let matcher_e = serde_json::to_value(matcher_e).unwrap();
            let query_parameters = vec![(String::from("c"), matcher_c), (String::from("e"), matcher_e)];
            let queries = HttpQueryParamsStub {
                query_parameters: Some(Map::from_iter(query_parameters))
            };
            let stub = RequestStub { url, queries, ..Default::default() };
            let url = Url::from(&stub);
            assert_eq!(url.path(), "/api/exact");
            let mut queries = url.query_pairs();
            assert_eq!(queries.count(), 3);
            assert_eq!(queries.next().unwrap(), (Cow::Borrowed("a"), Cow::Borrowed("b")));
            assert_eq!(queries.next().unwrap(), (Cow::Borrowed("c"), Cow::Borrowed("d")));
            assert_eq!(queries.next().unwrap(), (Cow::Borrowed("e"), Cow::Borrowed("f")));
        }
    }

    impl From<HttpUrlStub> for RequestStub {
        fn from(url: HttpUrlStub) -> Self {
            Self { url, ..Default::default() }
        }
    }
}