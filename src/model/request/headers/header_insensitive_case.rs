use std::convert::TryFrom;
use std::str::FromStr;

use http_types::headers::HeaderName;
use wiremock::{Match, Request};

use crate::model::request::headers::Header;

fn header_case_insensitive(key: String, value: String) -> HeaderCaseInsensitiveMatcher {
    HeaderCaseInsensitiveMatcher(key, value)
}

pub struct HeaderCaseInsensitiveMatcher(String, String);

impl Match for HeaderCaseInsensitiveMatcher {
    fn matches(&self, request: &Request) -> bool {
        HeaderName::from_str(self.0.as_str()).ok()
            .and_then(|key| request.headers.get(&key))
            .map_or(false, |header_values| {
                header_values.iter()
                    .any(|it| it.to_string().eq_ignore_ascii_case(&self.1))
            })
    }
}

impl TryFrom<&Header> for HeaderCaseInsensitiveMatcher {
    type Error = anyhow::Error;

    fn try_from(header_matcher: &Header) -> anyhow::Result<Self> {
        header_matcher
            .value
            .as_ref()
            .filter(|it| it.case_insensitive.eq(&Some(true)))
            .and_then(|it| it.equal_to.as_ref())
            .map(|exact| header_case_insensitive(header_matcher.key.to_string(), exact.to_string()))
            .ok_or_else(|| anyhow::Error::msg("No exact header matcher found"))
    }
}
