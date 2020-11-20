use std::convert::TryFrom;
use std::ops::Not;
use std::str::FromStr;

use regex::Regex;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug, Default)]
pub struct RequestMatcherDto {
    pub key: String,
    pub value: Option<MatcherValueDto>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct MatcherValueDto {
    pub equal_to: Option<Value>,
    pub case_insensitive: Option<bool>,
    pub contains: Option<String>,
    pub matches: Option<Value>,
    pub does_not_match: Option<Value>,
}

impl RequestMatcherDto {
    pub fn is_exact_match(&self) -> bool {
        self.is_equal_to() && !self.is_case_insensitive() && !self.is_contains()
    }

    pub fn is_equal_to(&self) -> bool {
        self.value.as_ref().and_then(|v| v.equal_to.as_ref()).is_some()
    }

    pub fn is_case_insensitive(&self) -> bool {
        self.value.as_ref().and_then(|v| v.case_insensitive).unwrap_or_default()
    }

    pub fn is_contains(&self) -> bool {
        self.value.as_ref()
            .and_then(|v| v.contains.as_ref())
            .map(|it| !it.is_empty())
            .unwrap_or_default()
            && !self.is_equal_to()
    }

    pub fn is_by_regex(&self) -> bool {
        let by_regex = self.is_matches() || self.is_does_not_matches();
        let by_equality = self.is_equal_to() || self.is_case_insensitive();
        let by_contains = self.is_contains();
        by_regex && by_equality.not() && by_contains.not()
    }

    pub fn is_matches(&self) -> bool {
        self.value.as_ref().and_then(|v| v.matches.as_ref()).is_some()
    }

    pub fn is_does_not_matches(&self) -> bool {
        self.value.as_ref().and_then(|v| v.does_not_match.as_ref()).is_some()
    }

    pub fn equal_to_as_str(&self) -> Option<String> {
        self.value.as_ref()
            .and_then(|it| it.equal_to.as_ref())
            .and_then(|v| {
                v.as_str().map(ToString::to_string)
                    .or_else(|| v.as_bool().map(|b| b.to_string()))
                    .or_else(|| v.as_i64().map(|i| i.to_string()))
            })
    }

    pub fn matches_as_regex(&self) -> Option<Regex> {
        self.value.as_ref()
            .and_then(|it| it.matches.as_ref())
            .and_then(|v| v.as_str())
            .and_then(|it| Regex::from_str(it).ok())
    }

    pub fn does_not_match_as_regex(&self) -> Option<Regex> {
        self.value.as_ref()
            .and_then(|it| it.does_not_match.as_ref())
            .and_then(|v| v.as_str())
            .and_then(|it| Regex::from_str(it).ok())
    }
}

impl TryFrom<(&String, &Value)> for RequestMatcherDto {
    type Error = anyhow::Error;

    fn try_from((k, v): (&String, &Value)) -> anyhow::Result<Self> {
        Ok(Self {
            key: k.to_owned(),
            value: serde_json::from_value(v.to_owned()).ok(),
        })
    }
}