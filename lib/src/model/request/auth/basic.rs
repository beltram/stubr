use std::str::FromStr;

use http_types::headers::HeaderName;
use serde::Deserialize;
use wiremock::{Match, Request};

#[derive(Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct BasicAuthDto {
    username: String,
    password: String,
}

pub struct BasicAuthMatcher(String);

impl BasicAuthMatcher {
    const AUTHORIZATION_HEADER: &'static str = "authorization";
    const BASIC_PREFIX: &'static str = "Basic";
}

impl Match for BasicAuthMatcher {
    fn matches(&self, req: &Request) -> bool {
        HeaderName::from_str(Self::AUTHORIZATION_HEADER).ok()
            .and_then(|k| req.headers.get(&k))
            .map(|v| v.as_str() == self.0.as_str())
            .unwrap_or_default()
    }
}

impl From<&BasicAuthDto> for BasicAuthMatcher {
    fn from(dto: &BasicAuthDto) -> Self {
        let value = base64::encode(format!("{}:{}", dto.username, dto.password));
        Self(format!("{} {}", Self::BASIC_PREFIX, value))
    }
}